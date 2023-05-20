
use crate::imagefilter;


use crate::utils::rgb_to_hsl;

pub struct Mask{

    pub widgetid:String,
    maskmethod:MaskFilter,
    mask_min:f32,
    mask_max:f32,
}

#[derive(Debug,Clone, Copy, PartialEq)]
pub enum MaskFilter{
    Hue,
    Vibrance,
    Saturation,
    Red,
    Green,
    Blue,
}
impl std::fmt::Display for MaskFilter {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"{:?}",self)
    }
}

impl Default for Mask {
    fn default() -> Self {
        Mask { widgetid: "a".to_string(), mask_min: 0.0, mask_max: 255.0, maskmethod:MaskFilter::Vibrance }
    }
}

impl Mask{
    pub fn make_with_id(id:String)->Self{
        Mask { widgetid: id, mask_min: 0.0, mask_max: 255.0, maskmethod:MaskFilter::Vibrance }
    }
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
                //let lum=get_luminance(pix);
                let lum=match self.maskmethod {
                    MaskFilter::Vibrance=>rgb_to_hsl(&*pix).2,
                    MaskFilter::Hue=>rgb_to_hsl(&*pix).0,
                    MaskFilter::Saturation=>rgb_to_hsl(&*pix).1,

                    //a.0 = rgb array a.0[0] =red
                    MaskFilter::Red=>pix.0[0] as f32,
                    MaskFilter::Green=>pix.0[1] as f32,
                    MaskFilter::Blue=>pix.0[2] as f32,
                    
                };

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
        
        let min_selector:f32;
        let max_selector:f32;

        match self.maskmethod {
            MaskFilter::Vibrance=>{min_selector=0.0;max_selector=1.0},
            MaskFilter::Hue=>{min_selector=0.0;max_selector=360.0},
            MaskFilter::Saturation=>{min_selector=0.0;max_selector=1.0}

            //a.0 = rgb array a.0[0] =red
            MaskFilter::Red=>{min_selector=0.0;max_selector=255.0},
            MaskFilter::Green=>{min_selector=0.0;max_selector=255.0},
            MaskFilter::Blue=>{min_selector=0.0;max_selector=255.0},
            
        }



        ui.heading("mask");
        ui.add(egui::Slider::new(&mut self.mask_min, min_selector..=max_selector).text("min"));
        ui.add(egui::Slider::new(&mut self.mask_max, self.mask_min..=max_selector).text("max"));
                
        ui.push_id(&self.widgetid, |ui|{ 
            egui::ComboBox::from_label(format!("sorting method {}",self.widgetid))
               .selected_text(format!("{}",self.maskmethod))
               .show_ui(ui, |ui|{
                   ui.selectable_value(&mut self.maskmethod, MaskFilter::Vibrance, "vib");
                   ui.selectable_value(&mut self.maskmethod, MaskFilter::Hue, "hue");
                   ui.selectable_value(&mut self.maskmethod, MaskFilter::Saturation, "saturation");

                   ui.selectable_value(&mut self.maskmethod, MaskFilter::Red, "red");
                   ui.selectable_value(&mut self.maskmethod, MaskFilter::Green, "green");
                   ui.selectable_value(&mut self.maskmethod, MaskFilter::Blue, "blue");
           });
        });
    }

}

