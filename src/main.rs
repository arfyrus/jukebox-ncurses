extern crate ncurses;
use ncurses::*;

struct Song {
    title: String,
    length: u32,
    author: String,
}

impl Song {
    fn new(title: String, length: u32, author: String) -> Song {
        Song {
            title,
            length,
            author,
        }
    }
    fn formal_length(&self) -> (u8, u32) {
        ((self.length / 60).try_into().unwrap(), self.length % 60)
    }
}

fn main() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    const REGULAR_PAIR: i16 = 0;
    const HIGHLIGHT_PAIR: i16 = 1;
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let song_label = "SONGS".to_string();
    let playing_label = "QUEUE".to_string();
    let space = 3;
    let divider = "-";

    let mut curr_y: usize = 0;
    let song_list = [
        ("Breaking Bad Theme", 76, "Dave Porter"),
        ("Pick Yourself Up", 192, "Nat \"King\" Cole"),
        ("Baby Blue", 217, "Badfinger"),
        ("Tuyo", 152, "Rodrigo Amarante"),
        ("Escape (The Pina Colada Song)", 276, "Rupert Holmes"),
        ("Al Compas De Mi Caballo", 215, "Los Imperials"),
        ("Negro y Azul", 205, "Los Cuates de Sinaloa"),
        ("A Dreamer's Holiday", 160, "Buddy Clark"),
        ("El Paso", 259, "Marty Robbins"),
        ("POP!", 168, "Im Nayeon"),
        ("The Feels", 198, "TWICE"),
        (
            "Crystal Blue Persuasion",
            242,
            "Tommy James & The Shondells",
        ),
        ("Dos Gardenias", 182, "Angel Canales"),
        ("Los Pistoleros", 210, "Jonaty Garcia"),
    ];
    let mut songs: Vec<Song> = vec![];

    for (title, length, author) in song_list {
        songs.push(Song::new(title.to_string(), length, author.to_string()));
    }

    let mut now_playing: Vec<Song> = Vec::new();
    loop {
        clear();
        let mut title_w: usize =
            std::cmp::max(song_label.chars().count(), playing_label.chars().count()) + 2;
        let mut author_w: usize = 0;
        let time_w: u8 = 6;
        let mut total_time = 0;

        // Find longest title and author of each song in songs and now_playing
        for song in songs.iter() {
            title_w = std::cmp::max(song.title.chars().count(), title_w);
            author_w = std::cmp::max(song.author.chars().count(), author_w);
        }
        for song in now_playing.iter() {
            title_w = std::cmp::max(song.title.chars().count(), title_w);
            author_w = std::cmp::max(song.author.chars().count(), author_w);
            total_time += song.length;
        }

        //Print label for song list, $songs
        {
            addch(ACS_ULCORNER());
            addch(ACS_HLINE());
            let label = format!(" [ {} ] ", song_label);
            addstr(&label);
            for _ in label.chars().count()..(title_w + author_w + time_w as usize + space * 2) {
                addch(ACS_HLINE());
            }
            addch(ACS_HLINE());
            addch(ACS_URCORNER());
        }

        // Prints song list, $songs
        for (index, song) in songs.iter().enumerate() {
            mv((index + 1).try_into().unwrap(), 0);
            let pair = if index == curr_y {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            };
            addch(ACS_VLINE());
            attron(COLOR_PAIR(pair));
            addstr(&format!(
                " {0:<title_w$}{3:^space$}{1:<author_w$}{3:^space$}({2}) ",
                song.title,
                song.author,
                {
                    let (mins, secs) = song.formal_length();
                    format!("{mins:0>1}:{secs:0>2}")
                },
                divider
            ));
            attroff(COLOR_PAIR(pair));
            addch(ACS_VLINE());
        }

        // Print label for queue, $now_playing
        {
            mv((songs.len() + 1).try_into().unwrap(), 0);
            addch(ACS_LTEE());
            addch(ACS_HLINE());
            let label = format!(" [ {} ] ", playing_label);
            addstr(&label);
            for _ in label.chars().count()..(title_w + author_w + time_w as usize + space * 2) {
                addch(ACS_HLINE());
            }
            addch(ACS_HLINE());
            addch(ACS_RTEE());
            addstr(&format!("\n"));
        }

        // Print queue, $now_playing
        for (index, song) in now_playing.iter().enumerate() {
            mv((index + songs.len() + 2).try_into().unwrap(), 0);
            let pair = if curr_y >= songs.len() && index == curr_y - songs.len() {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            };
            addch(ACS_VLINE());
            attron(COLOR_PAIR(pair));
            addstr(&format!(
                " {0:<title_w$}{3:^space$}{1:<author_w$}{3:^space$}({2}) ",
                song.title,
                song.author,
                {
                    let (mins, secs) = song.formal_length();
                    format!("{mins:0>1}:{secs:0>2}")
                },
                divider
            ));
            attroff(COLOR_PAIR(pair));
            addch(ACS_VLINE());
            addstr(&format!("\n"));
        }

        //Print label-less divider
        {
            addch(ACS_LTEE());
            for _ in 0..(title_w + author_w + space * 2 + 8) {
                addch(ACS_HLINE());
            }
            addch(ACS_RTEE());
            addstr("\n");
        }

        //Print songs in queue
        if now_playing.len() <= 0 {
            let message = "No songs in queue";
            addch(ACS_VLINE());
            addstr(&format!(
                " {message:<s1$} ",
                s1 = title_w + author_w + time_w as usize + space * 2
            ));
            addch(ACS_VLINE());
            addstr("\n");
        } else {
            let message = "Songs in queue";
            addch(ACS_VLINE());
            addstr(&format!(
                " {message}:{num:>s1$} ",
                num = now_playing.len(),
                s1 = title_w + author_w + time_w as usize + space * 2 - 1 - message.chars().count()
            ));
            addch(ACS_VLINE());
            addstr("\n");
        }

        //Print total time
        addch(ACS_VLINE());
        addstr(&format!(
            " {message:<m_width$} {time} ",
            message = "Total time: ",
            m_width = title_w + author_w + space * 2 - 4,
            time = format!(
                "({}:{:0>2}:{:0>2})",
                total_time / 3600,
                total_time % 3600 / 60,
                total_time % 60
            ),
        ));
        addch(ACS_VLINE());
        addstr(&format!("\n"));

        //Print help message
        {
            let message = "Press `?` for help";
            addch(ACS_VLINE());
            addstr(&format!(
                " {message:<s1$} ",
                s1 = title_w + author_w + time_w as usize + space * 2
            ));
            addch(ACS_VLINE());
            addstr("\n");
        }

        //Print ending border
        {
            addch(ACS_LLCORNER());
            addch(ACS_HLINE());
            for _ in 0..(title_w + author_w + time_w as usize + space * 2) {
                addch(ACS_HLINE());
            }
            addch(ACS_HLINE());
            addch(ACS_LRCORNER());
        }

        let choice = getch();
        match choice as u8 as char {
            'q' => break,
            'k' => {
                if curr_y > 0 {
                    curr_y -= 1;
                }
            }
            'j' => {
                if curr_y < songs.len() + now_playing.len() - 1 {
                    curr_y += 1;
                }
            }
            '\n' => {
                let in_song = curr_y < songs.len();
                if in_song {
                    now_playing.push(songs.remove(curr_y));
                } else {
                    songs.push(now_playing.remove(curr_y - songs.len()));
                }
                if in_song && curr_y == songs.len() && songs.len() > 0 {
                    curr_y = songs.len() - 1;
                } else if !in_song && curr_y == songs.len() - 1 {
                    curr_y = songs.len();
                }
                if curr_y >= songs.len() + now_playing.len() {
                    curr_y = songs.len() + now_playing.len() - 1;
                }
            }
            '\t' => {
                curr_y = {
                    if curr_y < songs.len() && now_playing.len() > 0 {
                        songs.len()
                    } else if curr_y >= songs.len() && songs.len() > 0 {
                        0
                    } else {
                        curr_y
                    }
                };
            }
            '?' => {
                clear();
                use std::collections::HashMap;
                let controls = HashMap::from([
                    ("k/j", "Move up and down"),
                    ("TAB", "Switch between song list and queue"),
                    ("q", "Quit program"),
                    ("ENTER", "Add/remove highlighted element from queue"),
                ]);

                let (mut long_key, mut long_value) = (0, 0);

                for (key, value) in &controls {
                    long_key = std::cmp::max(long_key, key.chars().count());
                    long_value = std::cmp::max(long_value, value.chars().count());
                }

                let label = " [ CONTROLS ] ";
                addch(ACS_ULCORNER());
                addch(ACS_HLINE());
                addstr(&label);
                for _ in label.chars().count()..(long_key + long_value + space + 1) {
                    addch(ACS_HLINE());
                }
                addch(ACS_URCORNER());
                addstr("\n");

                for (key, value) in &controls {
                    addch(ACS_VLINE());
                    addstr(&format!(
                        " {key:<long_key$}{divider:^space$}{value:<long_value$} "
                    ));
                    addch(ACS_VLINE());
                    addstr("\n");
                }
                {
                    addch(ACS_LTEE());
                    for _ in 0..(long_key + long_value + space + 2) {
                        addch(ACS_HLINE());
                    }
                    addch(ACS_RTEE());
                    addstr("\n");
                    let message = "Press any key to exit this menu";
                    addch(ACS_VLINE());
                    addstr(&format!(
                        " {message:<s1$} ",
                        s1 = long_key + long_value + space
                    ));
                    addch(ACS_VLINE());
                    addstr("\n");
                }
                addch(ACS_LLCORNER());
                addch(ACS_HLINE());
                for _ in 0..(long_key + long_value + space + 1) {
                    addch(ACS_HLINE());
                }
                addch(ACS_LRCORNER());
                getch();
            }
            _ => {}
        }
    }
    endwin();
}
