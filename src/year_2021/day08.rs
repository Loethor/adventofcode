


pub fn run(input: Vec<String>) {

    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 367);


    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 94813675);

}

fn solve_part1(input: &Vec<String>) -> i32 {
    let mut count = 0;
    for line in input{
        let mut iterator = line.split(" | ");
        
        // discard first half of the line
        iterator.next().unwrap();
        for element in iterator.next().unwrap().split(" "){
            // println!("{}",element);
            match element.chars().count() {
                2 => count +=1,
                3 => count +=1,
                4 => count +=1,
                7 => count +=1,
                _ => count +=0,

            }
        }
    }
    count
}

fn solve_part2(input: &Vec<String>) -> i32 {
    let mut count = 0;
    for line in input{
        let mut iterator = line.split(" | ");
        let input_segments = iterator.next().unwrap();
        let array_of_input_segments:Vec<String> = input_segments.split(" ").map(|s| s.to_string()).collect();
        
        let mut translator:Segments = Segments::new();

        process_input_segments(&mut translator, array_of_input_segments);

        let out_segments = iterator.next().unwrap();
        let array_of_output_segments:Vec<String> = out_segments.split(" ").map(|s| s.to_string()).collect();
        let mut out_numb:Vec<i32> = Vec::new();
        for word in array_of_output_segments{

            let big_number = translator.big_number(word);
            println!("{}", big_number);
            out_numb.push(big_number);
        }

        //TODO .to_digit(10)
        count += 1000*out_numb[0] + 100*out_numb[1] + 10*out_numb[2] + out_numb[3];

    }
    count
}


fn process_input_segments(tr: &mut Segments, array_of_input_segments:Vec<String>){ 
    let mut one:String = String::new();
    let mut seven:String = String::new();
    let mut four:String = String::new();
    let mut five_segments_numbers:Vec<String> = Vec::new();
    for element in array_of_input_segments{
        match element.chars().count() {
            2 => one = element,
            3 => seven = element,
            4 => four = element,
            5 => five_segments_numbers.push(element),
            _ => continue,

        }
    }


    // with 1 obtain a0 from 7
    for letter in seven.chars(){
        if !one.contains(letter){
            tr.a0 = letter;
        }
    }
    
    // with 1 obtain 3 from the 5 digits
    let mut three = String::new();
    'outer: for five_digit_number in &mut five_segments_numbers{
        for letter in one.chars(){
            if !five_digit_number.contains(letter){
                continue 'outer;
            }
        }
        three = five_digit_number.to_string();
    }
    let index = five_segments_numbers.iter().position(|x| *x == three).unwrap();
    five_segments_numbers.remove(index);


    // with 4 obtain 2 from the 5 digits
    let mut two = String::new();
    'outer: for five_digit_number in &mut five_segments_numbers{
        let mut count = 0;
        for letter in four.chars(){
            if !five_digit_number.contains(letter){
                count +=1;
            }
            if count == 2{
                two = five_digit_number.to_string();
                break 'outer;
            }
        }
    }
    let index = five_segments_numbers.iter().position(|x| *x == two).unwrap();
    five_segments_numbers.remove(index);

    // 5 is the remaining
    let five = five_segments_numbers[0].to_string();
    
    // obtain f0 with 5 and 1
    for letter in one.chars(){
        if five.contains(letter){
            tr.f0 = letter;
        }
    }


    // obtain c0 with 2 and 1
    for letter in one.chars(){
        if two.contains(letter){
            tr.c0 = letter;
        } 
    }

    // obtain b0 with 4 and 3
    for letter in four.chars(){
        if !three.contains(letter){
            tr.b0 = letter;
        }
    }

    // obtain d0 with 4 and b0, c0, f0
    for letter in four.chars(){
        if tr.b0 != letter && tr.c0 != letter && tr.f0 != letter{
            tr.d0 = letter;
        }
    }

    // obtain e0 with 3 and 2
    for letter in two.chars(){
        if !three.contains(letter){
            tr.e0 = letter;
        }
    }

    // obtain g0
    for letter in "abcdefg".chars(){
        if letter != tr.a0 && letter != tr.b0 && letter != tr.c0 && letter != tr.d0 && letter != tr.e0 && letter != tr.f0{
            tr.g0 = letter;
        } 
    }



    // println!("HOLAA {}", tr.translate('b'));
    



}


struct Segments {
    a0:char,
    b0:char,
    c0:char,
    d0:char,
    e0:char,
    f0:char,
    g0:char,
    
}

impl Segments {

    pub fn new() -> Self {
        Self { a0:'t',b0:'t',c0:'t',d0:'t',e0:'t',f0:'t',g0:'t' }
    }

    fn translate(&self, c_i:char) -> char{

        if c_i == self.a0{
            'a'
        } else if c_i == self.b0{
            'b' 
        } else if c_i == self.c0{
            'c' 
        } else if c_i == self.d0{
            'd' 
        } else if c_i == self.e0{
            'e' 
        } else if c_i == self.f0{
            'f' 
        } else {
            'g' 
        }

        // match c_i {
        //     self.a0 => 'a',
        //     b0 => 'b',
        //     c0 => 'c',
        //     d0 => 'd',
        //     e0 => 'e',
        //     f0 => 'f',
        //     g0 => 'g',
        // }
    }

    fn big_number(&self, input_string:String) -> i32{

        // 8 abcdefg   7digits
        if  input_string.contains(self.a0) && 
            input_string.contains(self.b0) && 
            input_string.contains(self.c0) && 
            input_string.contains(self.d0) && 
            input_string.contains(self.e0) && 
            input_string.contains(self.f0) && 
            input_string.contains(self.g0){
            return 8;
        } 
        // 0 abcefg   6digits
        else if input_string.contains(self.a0) && 
                input_string.contains(self.b0) && 
                input_string.contains(self.c0) && 
                input_string.contains(self.e0) && 
                input_string.contains(self.f0) && 
                input_string.contains(self.g0){
            return 0;
        } 
        // 6 abdefg   6digits
        else if input_string.contains(self.a0) && 
                input_string.contains(self.b0) && 
                input_string.contains(self.d0) && 
                input_string.contains(self.e0) && 
                input_string.contains(self.f0) && 
                input_string.contains(self.g0){
        return 6;
        } 
        // 9 abcdfg   6digits
        else if input_string.contains(self.a0) && 
                input_string.contains(self.b0) && 
                input_string.contains(self.c0) && 
                input_string.contains(self.d0) && 
                input_string.contains(self.f0) && 
                input_string.contains(self.g0){
        return 9;
        } 
        // 5 abdfg   5digits
        else if input_string.contains(self.a0) && 
                input_string.contains(self.b0) && 
                input_string.contains(self.d0) && 
                input_string.contains(self.f0) && 
                input_string.contains(self.g0){
        return 5;
        } 
        // 2 abdfg   5digits
        else if input_string.contains(self.a0) && 
                input_string.contains(self.c0) && 
                input_string.contains(self.d0) && 
                input_string.contains(self.e0) && 
                input_string.contains(self.g0){
        return 2;
        } 
        // 3 acdfg   5digits
        else if input_string.contains(self.a0) && 
                input_string.contains(self.c0) && 
                input_string.contains(self.d0) && 
                input_string.contains(self.f0) && 
                input_string.contains(self.g0){
        return 3;
        }
        // 4 abdfg   4digits
        else if input_string.chars().count() == 4{
            return 4;
        }
        else if input_string.chars().count() == 3{
            return 7;
        }
        else if input_string.chars().count() == 2{
            return 1;
        } else {
            return -1;
        }
    }
}