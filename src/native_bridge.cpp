// SPDX-License-Identifier: GPL-3.0-or-later
// FXMP native in-process adapter; original Dock engine is attributed separately.
#include "App.h"
#include <windows.h>
#include <thread>
#include <mutex>
#include <chrono>
#include <string>

extern int WINAPI wWinMain(HINSTANCE, HINSTANCE, PWSTR, int);
namespace {
std::mutex lock;
std::thread worker;
HWND own_control() {
    HWND h = FindWindowW(L"FxmpDockCtrl", nullptr);
    if (!h) return nullptr;
    DWORD pid = 0;
    GetWindowThreadProcessId(h, &pid);
    return pid == GetCurrentProcessId() ? h : nullptr;
}
bool start_locked() {
    if (own_control()) return true;
    if (worker.joinable()) worker.join();
    worker = std::thread([] {
        std::wstring args = L"--fxmp-mode";
        wWinMain(GetModuleHandleW(nullptr), nullptr, args.data(), SW_HIDE);
    });
    for (int i = 0; i < 200; ++i) {
        if (own_control()) return true;
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
    return false;
}
bool send_command(UINT msg) {
    std::lock_guard<std::mutex> guard(lock);
    if (!start_locked()) return false;
    return PostMessageW(own_control(), msg, 0, 0) != FALSE;
}
} // namespace

extern "C" bool dock_native_start() {
    std::lock_guard<std::mutex> guard(lock);
    return start_locked();
}
extern "C" bool dock_native_show() { return send_command(dk::WM_APP_SHOW); }
extern "C" bool dock_native_hide() { return send_command(dk::WM_APP_HIDE); }
extern "C" bool dock_native_toggle() { return send_command(dk::WM_APP_TOGGLE); }
extern "C" bool dock_native_open_settings() { return send_command(dk::WM_APP_SHOWSETTINGS); }
extern "C" bool dock_native_is_visible() {
    const HWND h = FindWindowW(L"DockOverlay", nullptr);
    DWORD pid = 0;
    if (h) GetWindowThreadProcessId(h, &pid);
    return h && pid == GetCurrentProcessId() && IsWindowVisible(h);
}
extern "C" bool dock_native_set_monitor(const wchar_t* value) {
    if (!value) return false;
    std::lock_guard<std::mutex> guard(lock);
    if (!start_locked()) return false;
    HWND h = own_control();
    COPYDATASTRUCT data{
        dk::WM_FXMP_SET_MONITOR,
        static_cast<DWORD>((wcslen(value) + 1) * sizeof(wchar_t)),
        const_cast<wchar_t*>(value)
    };
    return SendMessageW(h, WM_COPYDATA, 0, reinterpret_cast<LPARAM>(&data)) != FALSE;
}
extern "C" bool dock_native_stop() {
    std::lock_guard<std::mutex> guard(lock);
    HWND h = own_control();
    if (h && !PostMessageW(h, WM_CLOSE, 0, 0)) return false;
    if (worker.joinable()) worker.join();
    return true;
}
