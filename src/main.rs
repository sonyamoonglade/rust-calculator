use std::fmt;

struct StdinReader {
   stdin: std::io::Stdin,
}


#[derive(Debug)] 
enum Command {
    Sum,
    Diff,
    Div,
    Mult
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
        Command::Sum => write!(f, "+"),
        Command::Div => write!(f, "/"),
        Command::Mult => write!(f, "*"),
        Command::Diff => write!(f, "-"),
        }
    }
}

#[derive(Debug)] 
struct Expression {
    v1: f64,
    v2: f64,
    command: Command
}

impl Expression{
    fn new(v1: f64, v2: f64, c:Command) -> Expression{
        return Expression { v1, v2, command: c }
    }
}

impl StdinReader {

    fn new() -> StdinReader {
       return StdinReader { stdin: std::io::stdin()};
    }

    fn start_loop(&self){
        loop {
            let mut input: String = String::new();
            if self.stdin.read_line(&mut input).is_err(){
                println!("internal error!");
                continue;       
            }

            if input.contains("end") {
                break;
            }

            let expr: Result<Expression, String> = Self::as_expression(&input.as_str());
            if expr.is_err(){
                println!("{}", expr.as_ref().unwrap_err());
                continue;
            }

            let result: f64 = Self::calculate(expr.unwrap());
            println!("result: {:.1$}", result, 2usize);
        }
    }

    fn calculate(e: Expression) -> f64 {
        match e.command {
            Command::Sum => e.v1 + e.v2,
            Command::Div => e.v1 / e.v2,
            Command::Mult => e.v1 * e.v2,
            Command::Diff => e.v1 - e.v2,
        }
    }

    fn parse_int(s: &str) -> Result<f64, std::num::ParseFloatError>{
        return s.trim_end().parse::<f64>();
    }

    fn as_expression(s: &str) -> Result<Expression, String> {
      let result = s.split(' ')
          .map(|v| v.trim_end())
          .collect::<Vec<&str>>();
    
      if result.len() != 3{
          return Err("invalid number of input arguments!".to_string());
      }
        
      if !Self::is_sign(result[1]) {
        return Err("invalid sign passed".to_string());
      }

      let v1 = Self::parse_int(&result[0]);
      let v2 = Self::parse_int(&result[2]);
    
      if v1.is_err() || v2.is_err() {
       return Err("invalid numeric input".to_string());
      }
        
      Ok(Expression::new(v1.unwrap(), v2.unwrap(), Self::parse_sign(&result[1])))
    }

    fn is_sign(s: &str) -> bool {
        return ["+", "-", "/", "*"]
            .contains(&s)
    }

    fn parse_sign(s: &str) -> Command{
        match s{
            "+" => Command::Sum,
            "-" => Command::Diff,
            "/" => Command::Div,
            "*" => Command::Mult,
            _ => unreachable!()
        }
    }

}


fn main() {

    let stdin_reader = StdinReader::new();
    stdin_reader.start_loop();

}
