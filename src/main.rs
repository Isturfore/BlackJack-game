use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

fn init(rng: &mut impl Rng, vec: &mut Vec<i32>, pl_cards: &mut i32, dl_cards: &mut i32) {
    if let Some(&card) = vec.choose(rng) {
        *dl_cards += card;
        if let Some(pos) = vec.iter().position(|&x| x == card) {
            vec.remove(pos);
        }
    }
    if let Some(&card) = vec.choose(rng) {
        *pl_cards += card;
        if let Some(pos) = vec.iter().position(|&x| x == card) {
            vec.remove(pos);
        }
    }
}

fn dl_choose(rng: &mut ThreadRng, vec: &mut Vec<i32>, dl_cards: &mut i32) -> bool {
    if *dl_cards < 17 {
        if let Some(&card) = vec.choose(rng) {
            *dl_cards += card;
            if let Some(pos) = vec.iter().position(|&x| x == card) {
                vec.remove(pos);
            }
            println!("Дилер взял карту.");
            return true;
        }
    }
    println!("Дилер сделал пас.");
    return false;
}

fn pl_choose(rng: &mut ThreadRng, vec: &mut Vec<i32>, pl_cards: &mut i32) -> bool {
    if *pl_cards > 21 {
        println!("Сумма ваших карт > 21. Пас от вас");
        return false;
    }

    let mut input: String = String::new();
    println!("Сумма ваших карт: {}\n1 - Взять карту, 2 - Пас: ", pl_cards);

    std::io::stdin().read_line(&mut input).expect("Ошибка чтения.");
    let choice: i32 = input.trim().parse().expect("Это не число.");

    if choice == 1 || choice == 2 {
        if choice == 1 {
            if let Some(&card) = vec.choose(rng) {
                *pl_cards += card;
                if let Some(pos) = vec.iter().position(|&x| x == card) {
                    vec.remove(pos);
                }

                return true;
            }
        }
        else {
            return false;
        }
    }
    else {
        println!("Ошибка ввода. Пас");
        return false;
    }
    return false;
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let mut vec: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10,11];
    let mut pl_cards: i32 = 0;
    let mut dl_cards: i32 = 0;

    init(&mut rng, &mut vec, &mut pl_cards, &mut dl_cards);

    while !vec.is_empty() {
        let dl_res: bool = dl_choose(&mut rng, &mut vec, &mut dl_cards);
        let pl_res: bool = pl_choose(&mut rng, &mut vec, &mut pl_cards);

        if pl_cards > 21 {
            println!("Вы перебрали! Игра окончена.");
            break;
        }
        if dl_cards > 21 {
            println!("Дилер перебрал! Вы победили.");
            break;
        }

        if !dl_res && !pl_res {
            break;
        }
    }
    if (pl_cards > 21 && dl_cards > 21) || (pl_cards == dl_cards) {
        println!("Ничья. Количество карт дилера: {}, Ваше количество карт: {}", dl_cards, pl_cards);
    }
    else if pl_cards > dl_cards && pl_cards <= 21 {
        println!("Вы победили. Количество карт дилера: {}, Ваше количество карт: {}", dl_cards, pl_cards);
    }
    else if dl_cards > pl_cards && dl_cards <= 21 {
        println!("Победил дилер. Его количество карт: {}, Ваше количество карт: {}.", dl_cards, pl_cards);
    }
}
