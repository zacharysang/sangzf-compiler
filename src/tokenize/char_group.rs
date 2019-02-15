pub enum CharGroup {
  AlphaLower(char),
  AlphaUpper(char),
  Number(char),
  Other(char)
}


impl CharGroup {

  pub fn is_ws(ch: char) -> bool {
    return ch == ' ' || ch == '\t' || ch == '\n';
  }

  pub fn get(ch: char) -> CharGroup {
    match ch {
      'a' => CharGroup::AlphaLower('a'),
      'b' => CharGroup::AlphaLower('b'),
      'c' => CharGroup::AlphaLower('c'),
      'd' => CharGroup::AlphaLower('d'),
      'e' => CharGroup::AlphaLower('e'),
      'f' => CharGroup::AlphaLower('f'),
      'g' => CharGroup::AlphaLower('g'),
      'h' => CharGroup::AlphaLower('h'),
      'i' => CharGroup::AlphaLower('i'),
      'j' => CharGroup::AlphaLower('j'),
      'k' => CharGroup::AlphaLower('k'),
      'l' => CharGroup::AlphaLower('l'),
      'm' => CharGroup::AlphaLower('m'),
      'n' => CharGroup::AlphaLower('n'),
      'o' => CharGroup::AlphaLower('o'),
      'p' => CharGroup::AlphaLower('p'),
      'q' => CharGroup::AlphaLower('q'),
      'r' => CharGroup::AlphaLower('r'),
      's' => CharGroup::AlphaLower('s'),
      't' => CharGroup::AlphaLower('t'),
      'u' => CharGroup::AlphaLower('u'),
      'v' => CharGroup::AlphaLower('v'),
      'w' => CharGroup::AlphaLower('w'),
      'x' => CharGroup::AlphaLower('x'),
      'y' => CharGroup::AlphaLower('y'),
      'z' => CharGroup::AlphaLower('z'),
      'A' => CharGroup::AlphaUpper('A'),
      'B' => CharGroup::AlphaUpper('B'),
      'C' => CharGroup::AlphaUpper('C'),
      'D' => CharGroup::AlphaUpper('D'),
      'E' => CharGroup::AlphaUpper('E'),
      'F' => CharGroup::AlphaUpper('F'),
      'G' => CharGroup::AlphaUpper('G'),
      'H' => CharGroup::AlphaUpper('H'),
      'I' => CharGroup::AlphaUpper('I'),
      'J' => CharGroup::AlphaUpper('J'),
      'K' => CharGroup::AlphaUpper('K'),
      'L' => CharGroup::AlphaUpper('L'),
      'M' => CharGroup::AlphaUpper('M'),
      'N' => CharGroup::AlphaUpper('N'),
      'O' => CharGroup::AlphaUpper('O'),
      'P' => CharGroup::AlphaUpper('P'),
      'Q' => CharGroup::AlphaUpper('Q'),
      'R' => CharGroup::AlphaUpper('R'),
      'S' => CharGroup::AlphaUpper('S'),
      'T' => CharGroup::AlphaUpper('T'),
      'U' => CharGroup::AlphaUpper('U'),
      'V' => CharGroup::AlphaUpper('V'),
      'W' => CharGroup::AlphaUpper('W'),
      'X' => CharGroup::AlphaUpper('X'),
      'Y' => CharGroup::AlphaUpper('Y'),
      'Z' => CharGroup::AlphaUpper('Z'),
      '0' => CharGroup::Number('0'),
      '1' => CharGroup::Number('1'),
      '2' => CharGroup::Number('2'),
      '3' => CharGroup::Number('3'),
      '4' => CharGroup::Number('4'),
      '5' => CharGroup::Number('5'),
      '6' => CharGroup::Number('6'),
      '7' => CharGroup::Number('7'),
      '8' => CharGroup::Number('8'),
      '9' => CharGroup::Number('9'),
      _ => CharGroup::Other(ch)
      
    }
  }
}