

use crate::imagefilter;


pub mod mask;



#[derive(Debug,Clone, Copy, PartialEq)]
pub enum SortMethod{
    Hue,
    Vibrance,
    Saturation,
    Red,
    Green,
    Blue,
}


impl std::fmt::Display for SortMethod {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"{:?}",self)
    }
}



pub struct PixelSort{
    sorting_method: SortMethod,

    pub widgetid:String,

    min_mask:u8,
    max_mask:u8,

}


impl PixelSort{ 

    pub fn make_with_id(id:String)->Self {
        let mut asdf=Self::default();  
        asdf.widgetid=id;
        asdf
    }


    fn sort_pixel_vector(&self,vector: &mut Vec<image::Rgb<u8>>){
        match self.sorting_method {
            SortMethod::Vibrance=>vector.sort_by(|a,b| rgb_to_hsl(a).2.partial_cmp(&rgb_to_hsl(b).2).unwrap()),
            SortMethod::Hue=>vector.sort_by(|a,b| rgb_to_hsl(a).0.partial_cmp(&rgb_to_hsl(b).0).unwrap()),
            SortMethod::Saturation=>vector.sort_by(|a,b| rgb_to_hsl(a).1.partial_cmp(&rgb_to_hsl(b).1).unwrap()),

            //a.0 = rgb array a.0[0] =red
            SortMethod::Red=>vector.sort_by(|a,b| a.0[0].partial_cmp(&b.0[0]).unwrap()),
            SortMethod::Green=>vector.sort_by(|a,b| a.0[1].partial_cmp(&b.0[1]).unwrap()),
            SortMethod::Blue=>vector.sort_by(|a,b| a.0[2].partial_cmp(&b.0[2]).unwrap()),

        }
    }



}



impl Default for PixelSort{

    fn default() -> Self {
        Self {
            widgetid:"".to_string(),
            max_mask:250,
            min_mask:100,
            sorting_method:SortMethod::Vibrance,
        }
    }
}



fn get_luminance(pixel:&image::Rgb<u8>)->u8{
        let val1=(0.2126*pixel.0[0] as f32  ) as u8;
        let val2=(0.7152*pixel.0[1] as f32  ) as u8;
        let val3=(0.0722*pixel.0[2] as f32  ) as u8;
        let total = val1+val2+val3 ;
        total
}

fn rgb_to_hsl(pixel:&image::Rgb<u8>) -> (f32, f32, f32) {
    let (r, g, b) = (pixel.0[0] as f32 / 255.0, pixel.0[1] as f32 / 255.0, pixel.0[2] as f32 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let mut h = (max + min) / 2.0;
    let s ;
    let l = h;

    if max == min {
        h = 0.0;
        s = 0.0;
    } else {
        let d = max - min;
        s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

        match max {
            _ if max == r => h = (g - b) / d + (if g < b { 6.0 } else { 0.0 }),
            _ if max == g => h = (b - r) / d + 2.0,
            _ if max == b => h = (r - g) / d + 4.0,
            _ => (),
        }

        h /= 6.0;
    }

    (h, s, l)
}





//this does not get hue accurately.. at all but the results are kinda cool






impl imagefilter::ImageFilter for PixelSort{


    fn apply_filter(&mut self,image: image::ImageBuffer<image::Rgb<u8>,Vec<u8>>)->image::ImageBuffer<image::Rgb<u8>,Vec<u8>> {
        
//        let img = image;
        let mut asdf=image;
        
        let width= asdf.width();
        let height= asdf.height();

        //area definition
        //if we want to limit sort to a certain area of the screen
        let minx=0;
        let maxx=width;
        let miny=0;
        let maxy=height;

        let maskmin=self.min_mask;
        let maskmax=self.max_mask;

        //could possiubly be clone of asdf not just another intorgb
        let mut mask= asdf.clone();

        //loop all rows
        for y in miny..maxy{
            let mut buffer:Vec<image::Rgb<u8>>=vec![];

            //first and last changed value in the mask
            let mut mask_first:u32=99999999;
            let mut mask_last:u32=0;

            for x in minx..maxx{
                let pix=mask.get_pixel_mut(x, y);

                let lum=get_luminance(&*pix);

                //mask
                //we need to check first changed and last changed 
                if lum<maskmax && lum>maskmin { 
                    if mask_first==99999999{
                        mask_first=x;
                    }
                    *pix = image::Rgb([255,255,255]);
                    let pixel= asdf.get_pixel_mut(x, y);
                    buffer.push(*pixel);
                    mask_last=x;
                }else{
                    *pix = image::Rgb([0,0,0]);
                }

                //prob can be removed and moved to the if inside lum thing
            }

            //sort
            self.sort_pixel_vector(&mut buffer);

            //reassign
            let mut ind=0;
            for x in mask_first..mask_last {
                //mask pixel
                let mpix=mask.get_pixel_mut(x, y);
                if mpix.0[0]+mpix.0[1]+mpix.0[2]!=0{
                    let pixel= asdf.get_pixel_mut(x, y);
                    *pixel=buffer[ind];
                    ind+=1;
                }
            }
        }
        asdf
    }

    fn spawn_filter_widget(&mut self,ui:&mut egui::Ui) {
        ui.heading("Pixel Sorting");
        ui.add(egui::Slider::new(&mut self.min_mask, 0..=255).text("min"));
        ui.add(egui::Slider::new(&mut self.max_mask, self.min_mask..=255).text("max"));
                
        ui.push_id(&self.widgetid, |ui|{ 
            egui::ComboBox::from_label(format!("sorting method {}",self.widgetid))
               .selected_text(format!("{}",self.sorting_method))
               .show_ui(ui, |ui|{
                   ui.selectable_value(&mut self.sorting_method, SortMethod::Vibrance, "vib");
                   ui.selectable_value(&mut self.sorting_method, SortMethod::Hue, "hue");
                   ui.selectable_value(&mut self.sorting_method, SortMethod::Saturation, "saturation");

                   ui.selectable_value(&mut self.sorting_method, SortMethod::Red, "red");
                   ui.selectable_value(&mut self.sorting_method, SortMethod::Green, "green");
                   ui.selectable_value(&mut self.sorting_method, SortMethod::Blue, "blue");
           });
        });

        
    }


}



