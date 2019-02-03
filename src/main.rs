struct MonitorInfo 
{
    monitor_hight   : f64,
    monitor_width   : f64,
    diagonal        : f64,

    pixels_hight    : u32,
    pixels_width    : u32,
    pixels_per_inch : u32,
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

fn main() 
{
    let mut nfo : MonitorInfo = MonitorInfo::new(1920, 1080, 24.0);
    nfo.calc_monitor_values();
    nfo.print(); 
}
