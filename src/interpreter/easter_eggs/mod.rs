use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

static SATORU: &str = r#"
.............................................::...........!^.~.:....................................
............................................^!^:~........7J:.!7!....................................
.......................................^:..~!..^7......^?7:..^Y:....................................
......................................:?:.^7.::~!....^7?~....?~.....................................
......................................^?..?:.^7!:.:~7?!~~^^^Y?:::...................................
......................................:?.~Y..^J7!77~:..:!~777~~~~~~~~^:.............................
.................................:....:Y^7!~:77^^:..:::...........::^7?!!!~::.......................
............................:~!7!!!!~!757Y::~^.....^~~~~~~~^:........~!!???77!!777^.................
..........................~!!~^.......:!7~~^..........:~~^^~~!~~~~^:::~!7777!~~^^^^^^...............
........................~?7~^^:^7^.........^~^.........::^^~!!~:::^^~~^^:^^^^~!7J?~^:...............
.......................!?~::^~!~:.::........::..............^!??^.....::^~!?7!~^:...................
.......................~^~!7!^...:?:...................:..:!:.:!!:.........:.:^^~~~~~^^^:...........
...............:~~~~!!7!!~^::....~!....::.......~^....:?...^?:..^7:..^~...:!7?77!!77??7!~:..........
...............:~~~~^^::..^7.....!^....^?.......!!!7..:?..:.^?:..:7^.^Y!~!^!~!??^:::::..............
........................^!5^.....7:.....?^.~:...^?!5:.:?:^:7.~P~...7~.Y7!?J^~!^7?...................
.....................:~!7J7......7:.....~7.7!....?^Y!.:?~7.!! ??!^..7^!J!!!Y:^7:??..................
...................^~77!~?..~!...7:....^:7^:5:...7^7J:^Y^7..?^^?:!~..7!J^7^^5~^7.?!.................
................:!77!~:.J^..~7...!~..~!^?^J.7J:..!~:J!.!?7~.^7.?:.~!::??~.7~!5777:J^................
.................^^:...^J.!.:?...:7:.:?:7~!7.??:.~!.!J.^7!J:.7^!7..^?^:?7..7~7?J?7~J................
.......................7^~!..!!...^~..^7:?~J!.7Y::Y77P~:?.~?::7^57?777:??:^.!77?!?!?^...............
......................:J~7^...7^...!7..7^777J!.!?^?^:^?:?:.^J~7^5^..:^??7:!!.^J!?^?!:...............
......................??J??~:.:7^...7^.^!~~^J!!:.JB5PPPYY^..^?J??~YGPPG55~.~!.:?~:::................
.....................?P!~77J^^.:77:.:7..7^! :?^!J#&&&&&@BJ7..:!7Y#@&&&&&&BP~:!^^J~:.................
...................^!J^~?!??:5!?7??7^7~^YJY77?YY&&&&&&&&&PP5JJ?J#&&&&&&&&&5J..~~^~^.................
...................::.~7~:Y^??P^:??!!7J7YPPJ??7G&&&&&&&&&#G7:::!BB&&&&&&&&BP~...^^:.................
.........................!J!~?7!7!77J~^:.^^....:Y#&@@@@@&P!.....~JG#&&&&B5Y:........................
.........................Y!:.J^JJ:!~7?...........^!?JJJJ!:........:^!JJ!:~?.........................
.......................:!~...~Y?Y?^:!?7:.......................~^........77.........................
........................:.....^7J!J^:~^:.......................^^........?!.........................
..............................:?J?J~7^..................................^J~.........................
...............................~:57.:!!~~^..............................??:.........................
.................................?5...::~?7:..............^!~~~~7^.....!J~..........................
................................:!B:......^:..............^J!::~?:...:7?~...........................
.............................~PGB#&!..........:.............~!!!:..:7J!.............................
............................:5&&&&&5~:........~7!^............:..^7J!:..............................
............................~P&&&&&&&B5?~:.....^?PYJ7~^:........7J!:................................
............................!B&&&&&&&&&&&B5?~:.. ~7Y5G5Y?7!~~^^7!:..................................
............................7&&&&&&&&&&&&&&&&B5?~: .~JP5Y?J57^^:....................................
............................Y&&&&&&&&&&&&&&&&&&&&BP?~:^?55B#?^......................................
..........................^G&&&&&&&&&&&&&&&&&&&&&&&@&#PJ7?#&@#GY7^..................................
.........................~G@&&&&&&#PPG#&&&&&&&&&&&&&&&&@&#&&&&&@&&J.................................
........................?#&&&&&&&&&&#GP5PG#&&&&&&&&&&&&&&&&&&&&&&#~.................................
......................:5&&&&&&&&&&&&&&&&#GPPGB#&&&&&&&&&&&&&&&&&&J..................................
.....................7B@&&&&&&&&&&&&&&&&&&&&#BGGB#&&&&&&&&&&&&&&#:..................................
...................:5&&&&&&&&&&&&&&&&&&&&&&&&&&&##&&&&&&&&&&&&&&B!..................................
..................!B@&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&P^................................
.................?#&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&@#?:..............................
"#;

pub fn launch_satoru() -> ! {
    SATORU.split('\n').for_each(|line| {
        println!("{}", line);
        sleep(Duration::from_millis(50))
    });
    eprint!("Nah,");
    sleep(Duration::from_millis(300));
    eprintln!(" i'd win.");

    exit(0);
}