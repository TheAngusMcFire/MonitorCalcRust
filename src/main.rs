use std::env;
use std::error;
use std::fmt;

struct MonitorInfo 
{
    monitor_hight   : f64,
    monitor_width   : f64,
    diagonal        : f64,

    pixels_hight    : u32,
    pixels_width    : u32,
    pixels_per_inch : u32,
}

#[derive(Debug, Clone)]
struct GenericError
{
    msg : String
}

impl GenericError
{
    pub fn new(err_msg : String) -> GenericError
    {    
        return GenericError{msg : err_msg};
    } 
}

impl fmt::Display for GenericError 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for GenericError
{
    fn description(&self) -> &str 
    {
        "this is a generic error"
    }

    fn cause(&self) -> Option<&error::Error> 
    {
        None
    }
}

impl MonitorInfo
{
    pub fn new(px_hight : u32, px_width : u32, mon_diagonal : f64) -> MonitorInfo
    {
        return MonitorInfo 
        {
            monitor_hight   : 0.0,
            monitor_width   : 0.0,
            diagonal        : mon_diagonal,
    
            pixels_hight    : px_hight,
            pixels_width    : px_width,
            pixels_per_inch : 0
        }
    }

    pub fn print(&self)
    {
        println!("Monitor hight:    {}", self.monitor_hight   );
        println!("Monitor width:    {}", self.monitor_width   );
        println!("Monitor diagonal: {}", self.diagonal        );
        println!("Pixels hight:     {}", self.pixels_hight    );
        println!("Pixels width:     {}", self.pixels_width    );
        println!("Pixels per inch:  {}", self.pixels_per_inch );
    }

    pub fn calc_monitor_values(&mut self)
    {
        let pixel_size = self.diagonal / (((self.pixels_width * self.pixels_width) + (self.pixels_hight * self.pixels_hight)) as f64).sqrt();
        self.monitor_hight = pixel_size * (self.pixels_hight as f64);
        self.monitor_width = pixel_size * (self.pixels_width as f64);
    }
}

fn check_and_parse_args() -> Result<MonitorInfo, GenericError>
{
    let argc = env::args().len();

    if argc != 4
    {
        return Result::Err(GenericError::new("Error to less arguments provided".to_string()));
    }

    let args : Vec<String> = env::args().collect();

    let px_x : u32;
    let px_y : u32;
    let diag : f64;

    match args[1].parse::<u32>() 
    {
        Ok(result) => px_x = result, 
        Err(_err) => return Result::Err(GenericError::new("Error while parsing px_x value".to_string())),
    }

    match args[2].parse::<u32>() 
    {
        Ok(result) => px_y = result,
        Err(_err) => return Result::Err(GenericError::new("Error while parsing px_y value".to_string())),
    }

    match args[3].parse::<f64>() 
    {
        Ok(result) => diag = result,
        Err(_err) => return Result::Err(GenericError::new("Error while parsing diag value".to_string())),
    }

    let mut tmp = MonitorInfo::new(px_y, px_x, diag);
    tmp.calc_monitor_values();

    return Ok(tmp);
}

fn main() 
{
    let ret_code = check_and_parse_args();

    match ret_code
    {
        Ok(result) =>
        {
            result.print();
        }

        Err(error) =>
        {
            println!("{}",error);
        } 
    } 
}
