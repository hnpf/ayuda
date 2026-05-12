#include "include/hacker.hpp"
#include <ncurses.h>
#include <string>
#include <thread>
#include <chrono>

void breach(rust::Str target) {
    std::string t(target.data(), target.size());
    
    initscr();
    noecho();
    curs_set(0);
    start_color();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);
    attron(COLOR_PAIR(1));

    int row, col;
    getmaxyx(stdscr, row, col);

    mvprintw(row / 2 - 2, (col - 20) / 2, "breaching %s", t.c_str());
    refresh();
    std::this_thread::sleep_for(std::chrono::milliseconds(500));

    for (int i = 0; i <= 100; i += 5) {
        mvprintw(row / 2, (col - 50) / 2, "[");
        for (int j = 0; j < 48; j++) {
            if (j < (i * 48 / 100)) mvprintw(row / 2, (col - 50) / 2 + 1 + j, "#");
            else mvprintw(row / 2, (col - 50) / 2 + 1 + j, " ");
        }
        mvprintw(row / 2, (col - 50) / 2 + 49, "] %d%%", i);
        
        // random "matrix" text
        mvprintw(row / 2 + 2, (col - 30) / 2, "installing bonus opsec and overriding security protocol %d...", i * 1337);
        
        refresh();
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }

    mvprintw(row / 2 + 4, (col - 15) / 2, "access failed.. ");
    refresh();
    std::this_thread::sleep_for(std::chrono::seconds(1));

    endwin();
}
