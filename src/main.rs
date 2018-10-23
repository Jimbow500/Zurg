/*
TODO:

Make sub worlds like rooms that render on the map

*/

#![allow(unused)]
use std::io::{self, Write};
use std::{thread, time};
use std::process::exit;

extern crate colored;
use colored::*;


fn main(){
    println!("Welcome to Zurg");
    menu(false);
}

struct Stats {
    player_x: u32,
    player_y: u32,
    player_name: String,
    player_level: u32,
    player_id: u8,
}

struct Entity {
    //If yes it needs to be removed or garbage collected somehow.
    alive: bool,
    
    name: String,
    health: u32,
    level: u32,
    ent_id: u8,
    // How many units they move per a turn.
    speed: u32,
    
    x: u32,
    y: u32,
    
    // Can the player pass through?
    player_clip: bool,
    
    //Is it good or bad?
    attacts: bool,
    
    //Does it have gly-cock-mama?
    view_distance: u32,
}

struct Room {
    x: u32,
    y: u32,
    name: String,
    end_loot: bool,
    end_loot_item: u32,
    already_been: bool,
    level: u32,
}


fn printt(text_str: &str, wait: u64){

    //TODO change out all callings of println!() to printt();
    thread::sleep(time::Duration::from_millis(wait));
    println!("{}",text_str);
}

fn do_a_wait(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

fn menu(died: bool){
    loop {
    if died == true {println!("[0]Continue");};
    printt("[1]Load\n[2]New\n[3]Options\n[4]Help\n[5]Quit", 250);
    
    
    let mut input = askquest("What would you like todo?: ", 250);
    
    //TODO maybe figure out some way to make numbers work with ease
    if input.trim() == "1" {input = "load".to_string();};
    if input.trim() == "2" {input = "new".to_string();};
    if input.trim() == "3" {input = "option".to_string();};
    if input.trim() == "4" {input = "help".to_string();};
    if input.trim() == "5" {input = "quit".to_string();};
    
    
    // This is only enabled when the user dies
    if died == true {
    if input.trim() == "0" {input = "continue".to_string();};
    };
    
    //TODO remove .trim()
    if input.trim() == "load"{println!("This does not work currently"); continue;};
    if input.trim() == "new" {newgame(); continue;};
    if input.trim() == "option" {println!("This does not work currently"); continue;};
    if input.trim() == "help" {println!("uh don't die");}; //TODO make help bigger
    if input.trim() == "quit" {exit(1);};
    if input.trim() == "continue" {printt("um hold on", 250); continue;};
    println!("Sorry I didn't understand {}", input.to_ascii_uppercase());
    }; //end of loop
}

// This will load anymap with all the best items to test
fn dev_map_hub(){
    println!("Hello there where would you like to go?");
}

fn checkinv(inv: [u32; 64]){
    println!("You have..");
    for x in 0..64 {
    if inv[x] >= 1 {
        println!("{0}: {1}",item_index(x),inv[x]);
        }
    }
}

fn item_index(item_number: usize) -> String{

    let mut index: Vec<String> = vec![String::new(); 64];
    
    //TODO add a system that reads from either a text file or
    //be able to read from an external string array.
    index[0] = "Health".to_string();
    index[1] = "Mana".to_string();
    index[2] = "Armmor".to_string();
    index[3] = "Mintos".to_string();
    
    
    return index[item_number].clone();
}


fn give(inv: &mut [u32;64], item_number: usize, amount: u32){
    let item_name = item_index(item_number);
    inv[item_number] = amount;
    println!("You have gained {0} {1}", amount, item_name);
}

fn askquest(text: &str, wait: u64) -> String {
    thread::sleep(time::Duration::from_millis(wait));
    print!("{}", text);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input = input.to_ascii_lowercase();
    return input;
}

fn refresh_check(inv: &mut [u32; 64], stats: &mut Stats){
    
    // To see if the player has died
    if inv[0] == 0 {
        printt("You have died", 250);
        menu(true);
    }
    
    /*
    if stats[0] == 0 && stats[1] == 0 {
    printt("\nHello there, you've been sleeping some time now.\n", 500);
    
    let input = askquest("What's your name?: ", 250);
    
    if input.trim() == "map" {dev_map_hub();};
    
    
    let name = input.to_ascii_uppercase();
    do_a_wait(250);
    println!("I'm glad you can join us tonight {}", name);
    printt("I we we're walking around our camp grounds and we", 250);
    printt("happen to see your sleeping self over here.", 250);
    printt("DEBUG ERORR: you ran out of story for now you cuck <3", 250);
    
    };
    */
}

fn draw_map(stats: &mut Stats, world: &mut [u8; 262144]){
    
    let mut map_x: u32 = stats.player_x;
    let mut map_y: u32 = stats.player_y;
    
    //TODO fix this fucking mess

    for x in 0..16 {
    print!("{}", id_index(world[cord_to_int(map_x - 8, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x - 8, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x - 7, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x - 7, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x - 6, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x - 6, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x - 5, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x - 5, map_y + 7 - x)]));
    
    print!("{}", id_index(world[cord_to_int(map_x - 4, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x - 4, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x - 3, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x - 3, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x - 2, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x - 2, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x - 1, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x - 1, map_y + 7 - x)]));
    
    print!("{}", id_index(world[cord_to_int(map_x    , map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x    , map_y + 7 - x)]));
    
    print!("{}", id_index(world[cord_to_int(map_x + 1, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x + 1, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x + 2, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x + 2, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x + 3, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x + 3, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x + 4, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x + 4, map_y + 7 - x)]));
    
    print!("{}", id_index(world[cord_to_int(map_x + 5, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x + 5, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x + 6, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x + 6, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x + 7, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x + 7, map_y + 7 - x)]));
    print!("{}", id_index(world[cord_to_int(map_x + 8, map_y + 7 - x)]));   print!("{}", id_index(world[cord_to_int(map_x + 8, map_y + 7 - x)]));
    
    println!("");    
    };
    //io::stdout().flush().unwrap();
    
    
    /*
    for x in 0..257 {
    
        do_a_wait(0);
        print!("{}", id_index(world[cord_to_int(map_x, map_y)]));
        map_x += 1;
    
        if x % 16 == 0 && x > 0 {
            map_y += 1;
            println!("next");
            //io::stdout().flush().unwrap();
            
        };
    
    };
    */
}

fn id_index(id: u8) -> String {
    let mut index: Vec<String> = vec![String::new(); 64];
    
    index[0] = "█".white().to_string();
    index[1] = "█".red().to_string();
    index[2] = "█".black().to_string();
    
    return index[id as usize].clone();
}

fn cord_to_int(x: u32, y: u32) -> usize {
    return y as usize * 256 + x as usize;
}

fn set_ent_at(world: &mut [u8; 262144], x: u32, y: u32, id: u8) {
    let location: usize = cord_to_int(x ,y);
    world[location] = id;
}

fn render_ent(world: &mut [u8; 262144],x: u32, y: u32) {
    println!("{}", id_index(world[cord_to_int(x, y)]));
}

fn update_map(mut world: &mut [u8; 262144], stats: &mut Stats) {
    // Update player
    for x in 0..262144 {
        if world[x] == 1 {
            world[x] = 0;
        }
    }
    set_ent_at(&mut world, stats.player_x, stats.player_y, stats.player_id);
}

fn draw_line(mut world: &mut [u8; 262144], mut firstx: u32, mut firsty: u32, secondx: u32, secondy: u32, id: u8) {
        
    while firstx != secondx || firsty != secondy {
        
        set_ent_at(&mut world, firstx, firsty, id);
        
        if firstx > secondx {
            firstx -= 1;
        } else if firstx < secondx {
            firstx += 1;
        };
        
        if firsty > secondy {
            firsty -= 1;
        } else if firsty < secondy {
            firsty += 1;
        };
        
    };
}

fn player_input(input: String, world: &mut [u8; 262144], stats: &mut Stats, inv: &mut [u32; 64]) {
    
    let words: Vec<&str> = input.trim().split(' ').collect();

    // Find Verb
    for x in words.clone() {
        if x == "nice" {
            print!("Cool");
        } else if x == "kms" {
            exit(1);
        } else if x == "move" {
            // Find number
            for x in words.clone() {
                let unit = x.parse::<u32>();
                
                match {
                    Ok(var) => println!("we got a number {}", var),
                    Err(why) => why
                }
            };
        };
    };
    
    
}

fn newgame(){

    // Inv that will be passed around the game
    let mut inv: [u32; 64] = [0; 64];
    
    let mut stats = Stats { player_x: 37, player_y: 56, player_name: String::new(), player_level: 0, player_id: 1};
    
    let mut world: [u8; 262144] = [0; 262144];
    // Health isn't going to be in the stats because it will always be called
    // When you look at your invitory
    // give(&mut inv, 0, 100);
    inv[0] = 100;
    
    let mut input: String = String::new();
    
    // Borders of the world
    //TODO move these to the world gen
    draw_line(&mut world, 1, 1, 255, 1, 2);
    draw_line(&mut world, 1,   1, 1, 255,   2);
    draw_line(&mut world, 1, 255, 255, 255, 2);
    draw_line(&mut world, 255, 1, 255, 255, 2);
    
    
    loop {
        input = String::new();
        update_map(&mut world, &mut stats);
        draw_map(&mut stats, &mut world);
        input = askquest("-What would you like to do?: ", 250);
        
        player_input(input, &mut world, &mut stats, inv);
    };
    //TODO needs updating --> refresh_check(&mut inv, &mut stats);
    exit(1);
}

