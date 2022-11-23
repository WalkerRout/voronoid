#[derive(Debug, Clone)]
pub struct PPMImage {
  width: u32,
  height: u32,
  data: Vec<u32>,
}

impl PPMImage {
  pub fn new(width: u32, height: u32) -> PPMImage {
    let image = PPMImage {
      width,
      height,
      data: vec![0xFFFFFFFF; (width*height) as usize]
    };
    
    image
  }

  pub fn get_width(&self) -> u32 {
    self.width
  }

  pub fn get_height(&self) -> u32 {
    self.height
  }

  pub fn get_data(&self) -> Vec<u32> {
    self.data.clone()
  }

  pub fn get_data_at_index(&self, i: usize, j: usize) -> u32 {
    self.data[i * (self.width as usize) + j]
  }

  pub fn set_data_at_index(&mut self, i: usize, j: usize, colour: u32) {
    self.data[i * (self.width as usize) + j] = colour;
  }
  
  pub fn fill(&mut self, colour: u32) {
    self.data
      .iter_mut()
      .for_each(|ele| {
        *ele = colour;
      });
  }

  pub fn save(&mut self, file_path: &str) -> Result<(), std::io::Error> {
    use std::io::Write;
    
    let path = std::path::Path::new(file_path);
    let mut file = std::fs::File::create(&path)?;

    let header = format!("P6\n{} {}\n255\n", self.width, self.height);
    let mut body: Vec<u8> = Vec::new();

    for i in 0..self.width*self.height {
      let index = i as usize;
      
      let red   = (self.data[index] >> (8*3)) as u8;
      let green = (self.data[index] >> (8*2)) as u8;
      let blue  = (self.data[index] >> (8*1)) as u8;
      
      body.push(red);
      body.push(green);
      body.push(blue);
    }

    write!(file, "{}", header)?;
    file.write_all(&body[..])?;
    
    Ok(())
  }
  
}
