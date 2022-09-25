extern crate ncurses;
use ncurses::*;

struct Song {
    title: String,
    length: u32,
    author: String,
}

enum TimeType {
    Seconds,
    SecAndMins,
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

    let song_label = "Songs".to_string();
    let playing_label = "Queue".to_string();
    let space = 3;
    let divider = "=";

    let mut curr_y: usize = 0;
    let mut time_type: TimeType = TimeType::SecAndMins;
    let mut songs: Vec<Song> = vec![
        Song::new(
            "Breaking Bad Theme".to_string(),
            76,
            "Dave Porter".to_string(),
        ),
        Song::new(
            "Pick Yourself Up".to_string(),
            192,
            "Nat \"King\" Cole".to_string(),
        ),
        Song::new("Baby Blue".to_string(), 217, "Badfinger".to_string()),
        Song::new("Tuyo".to_string(), 152, "Rodrigo Amarante".to_string()),
        Song::new(
            "Escape (The Pina Colada Song)".to_string(),
            276,
            "Rupert Holmes".to_string(),
        ),
        Song::new(
            "Al Compas De Mi Caballo".to_string(),
            215,
            "Los Imperials".to_string(),
        ),
        Song::new(
            "Negro y Azul".to_string(),
            205,
            "Los Cuates de Sinaloa".to_string(),
        ),
        Song::new(
            "A Dreamer's Holiday".to_string(),
            160,
            "Buddy Clark".to_string(),
        ),
        Song::new("El Paso".to_string(), 259, "Marty Robbins".to_string()),
        Song::new("POP!".to_string(), 168, "Im Nayeon".to_string()),
        Song::new("The Feels".to_string(), 198, "TWICE".to_string()),
        Song::new(
            "Crystal Blue Persuasion".to_string(),
            242,
            "Tommy James & The Shondells".to_string(),
        ),
        Song::new(
            "Dos Gardenias".to_string(),
            182,
            "Angel Canales".to_string(),
        ),
        Song::new(
            "Los Pistoleros".to_string(),
            210,
            "Jonaty Garcia".to_string(),
        ),
    ];
    let mut now_playing: Vec<Song> = Vec::new();
    loop {
        clear();
        let mut title_w: usize =
            std::cmp::max(song_label.chars().count(), playing_label.chars().count()) + 2;
        let mut author_w: usize = 0;
        let time_w: u8 = match time_type {
            TimeType::Seconds => 5,
            TimeType::SecAndMins => 6,
        };
        let mut total_time = 0;
        for song in songs.iter() {
            title_w = std::cmp::max(song.title.chars().count(), title_w);
            author_w = std::cmp::max(song.author.chars().count(), author_w);
        }
        for song in now_playing.iter() {
            title_w = std::cmp::max(song.title.chars().count(), title_w);
            author_w = std::cmp::max(song.author.chars().count(), author_w);
            total_time += song.length;
        }
        addstr(&format!(
            "+{:-<1$}+",
            format!("-[ {} ]-", song_label),
            title_w + author_w + time_w as usize + space * 2 + 2
        ));
        for (index, song) in songs.iter().enumerate() {
            mv((index + 1).try_into().unwrap(), 0);
            let pair = if index == curr_y {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            };
            attron(COLOR_PAIR(pair));
            addstr(&format!(
                "| {0:<title_w$}{3:^space$}{1:<author_w$}{3:^space$}({2}) |",
                song.title,
                song.author,
                match time_type {
                    TimeType::Seconds => format!("{:>3}", song.length),
                    TimeType::SecAndMins => {
                        let (mins, secs) = song.formal_length();
                        format!("{mins:0>1}:{secs:0>2}")
                    }
                },
                divider
            ));
            attroff(COLOR_PAIR(pair));
        }
        mv((songs.len() + 1).try_into().unwrap(), 0);
        addstr(&format!(
            "+{:-<1$}+",
            format!("-[ {} ]-", playing_label),
            title_w + author_w + time_w as usize + space * 2 + 2
        ));
        for (index, song) in now_playing.iter().enumerate() {
            mv((index + songs.len() + 2).try_into().unwrap(), 0);
            let pair = if curr_y >= songs.len() && index == curr_y - songs.len() {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            };
            attron(COLOR_PAIR(pair));
            addstr(&format!(
                "| {0:<title_w$}{3:^space$}{1:<author_w$}{3:^space$}({2}) |",
                song.title,
                song.author,
                match time_type {
                    TimeType::Seconds => format!("{:>3}", song.length),
                    TimeType::SecAndMins => {
                        let (mins, secs) = song.formal_length();
                        format!("{mins:0>1}:{secs:0>2}")
                    }
                },
                divider
            ));
            attroff(COLOR_PAIR(pair));
        }
        mv((songs.len() + now_playing.len() + 2).try_into().unwrap(), 0);
        addstr(&format!(
            "+{:-<1$}+\n",
            "",
            title_w + author_w + time_w as usize + space * 2 + 2
        ));
        addstr(&format!(
            "| {message:<m_width$} {time:>t_width$} |\n",
            message = "Total time: ",
            m_width = title_w + author_w + space * 2 - 2,
            time = format!("({:>2}:{:0>2})", total_time / 60, total_time % 60),
            t_width = time_w as usize,
        ));
        addstr(&format!(
            "+{:-<1$}+\n",
            "",
            title_w + author_w + time_w as usize + space * 2 + 2
        ));
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
                time_type = match time_type {
                    TimeType::Seconds => TimeType::SecAndMins,
                    TimeType::SecAndMins => TimeType::Seconds,
                }
            }
            _ => {}
        }
    }
    endwin();
}
