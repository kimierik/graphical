
use crate::imagefilter;


pub struct Mask{

    pub widgetid:String,
    mask_min:u8,
    mask_max:u8,
}


impl Default for Mask {
    fn default() -> Self {
        Mask { widgetid: "a".to_string(), mask_min: 0, mask_max: 255 }
    }
}

impl Mask{
    pub fn make_with_id(id:String)->Self{
        Mask { widgetid: id, mask_min: 0, mask_max: 255 }
    }
}



fn get_luminance(pixel:&image::Rgb<u8>)->u8{
        let val1=(0.2126*pixel.0[0] as f32  ) as u8;
        let val2=(0.7152*pixel.0[1] as f32  ) as u8;
        let val3=(0.0722*pixel.0[2] as f32  ) as u8;
        let total = val1+val2+val3 ;
        total
}


impl imagefilter::ImageFilter for Mask{

    fn apply_filter(&mut self,image:image::ImageBuffer<image::Rgb<u8>,Vec<u8>>)->image::ImageBuffer<image::Rgb<u8>,Vec<u8>> {
        let mut asdf=image;
        
        let width= asdf.width();
        let height= asdf.height();

        //area definition
        //if we want to limit sort to a certain area of the screen
        let minx=0;
        let maxx=width;
        let miny=0;
        let maxy=height;

        let maskmin=self.mask_min;
        let maskmax=self.mask_max;

        for y in miny..maxy{
            for x in minx..maxx{
                let pix= asdf.get_pixel_mut(x, y);
                let lum=get_luminance(pix);

                if lum>maskmin && lum<maskmax {
                    *pix = image::Rgb([0,0,0]);
                }else{
                    *pix = image::Rgb([255,255,255]);
                }

            }
        }

        asdf
        
    }

    fn spawn_filter_widget(&mut self,ui:&mut egui::Ui) {
        
        ui.heading("mask");
        ui.add(egui::Slider::new(&mut self.mask_min, 0..=255).text("min"));
        ui.add(egui::Slider::new(&mut self.mask_max, self.mask_min..=255).text("max"));
                
    }

}

