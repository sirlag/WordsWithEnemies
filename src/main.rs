fn main() {
     attack("because","cause");
     attack("hello", "below");
     attack("hit", "miss");
     attack("rekt", "pwn");
     attack("combo", "jumbo");
     attack("critical", "optical");
     attack("isoenzyme", "apoenzyme");
     attack("tribesman", "brainstem");
     attack("blames", "nimble");
     attack("yakuza", "wizard");
     attack("longbow", "blowup");
 }

fn attack(l : &str, r : &str) -> u8{
      
    let mut left_side: Vec<char> = l.chars().collect();
    let mut right_side: Vec<char> = r.chars().collect();
    let mut left_side_clear = vec![];
    
    
    while left_side.len() != 0{
        let mut cleared = false;        
        let c = match left_side.pop() {
            Some(m) => m,
            None => ':'
        };
        
        for x in 0..right_side.len() {
            if right_side[x] == c {
                right_side[x] = ':';
                break;
            }
            cleared = x == right_side.len()-1;
        }
        if cleared {left_side_clear.push(c);};
    }
    
    let mut right_side_clear = vec![];
    
    
    while right_side.len() != 0 {
        let popped = match right_side.pop() {
            Some(m) => m,
            None => ':'
        };
        if popped != ':' {right_side_clear.push(popped);};
    }
        
    println!("Left Word : {}, Right Word {}", l, r);
    println!("Left Side has {} points, Right side has {} points.", left_side_clear.len(), right_side_clear.len());  
    return (left_side_clear.len() - right_side_clear.len()) as u8;
}