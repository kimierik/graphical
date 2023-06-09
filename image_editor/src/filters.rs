

use image::ImageBuffer;

use crate::imagefilter;


pub mod mask;
pub mod pixelate;
pub mod blur;


use crate::utils::rgb_to_hsl;

#[derive(Debug,Clone, Copy, PartialEq)]
pub enum SortMethod{
    Hue,
    Vibrance,
    Saturation,
    Red,
    Green,
    Blue,
}

#[derive(Debug,Clone, Copy, PartialEq)]
pub enum SortDirection{
    Vertical,
    Horizontal
}


impl std::fmt::Display for SortMethod {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"{:?}",self)
    }
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"{:?}",self)
    }
}


pub struct PixelSort{
    sorting_method: SortMethod,

    pub widgetid:String,

    min_mask:f32,
    max_mask:f32,

    sort_direction: SortDirection,
    invert_sort:bool,

}


impl PixelSort{ 

    pub fn make_with_id(id:String)->Self {
        let mut asdf=Self::default();  
        asdf.widgetid=id;
        asdf
    }


    fn sort_pixel_vector(&self,vector: &mut Vec<image::Rgb<u8>>){
        if !self.invert_sort{
            match self.sorting_method {
                SortMethod::Vibrance=>vector.sort_by(|a,b| rgb_to_hsl(a).2.partial_cmp(&rgb_to_hsl(b).2).unwrap()),
                SortMethod::Hue=>vector.sort_by(|a,b| rgb_to_hsl(a).0.partial_cmp(&rgb_to_hsl(b).0).unwrap()),
                SortMethod::Saturation=>vector.sort_by(|a,b| rgb_to_hsl(a).1.partial_cmp(&rgb_to_hsl(b).1).unwrap()),

                //a.0 = rgb array a.0[0] =red
                SortMethod::Red=>vector.sort_by(|a,b| a.0[0].partial_cmp(&b.0[0]).unwrap()),
                SortMethod::Green=>vector.sort_by(|a,b| a.0[1].partial_cmp(&b.0[1]).unwrap()),
                SortMethod::Blue=>vector.sort_by(|a,b| a.0[2].partial_cmp(&b.0[2]).unwrap()),

            }
        }else{
            match self.sorting_method {
                SortMethod::Vibrance=>vector.sort_by(|b,a| rgb_to_hsl(a).2.partial_cmp(&rgb_to_hsl(b).2).unwrap()),
                SortMethod::Hue=>vector.sort_by(|b,a| rgb_to_hsl(a).0.partial_cmp(&rgb_to_hsl(b).0).unwrap()),
                SortMethod::Saturation=>vector.sort_by(|b,a| rgb_to_hsl(a).1.partial_cmp(&rgb_to_hsl(b).1).unwrap()),

                //a.0 = rgb array a.0[0] =red
                SortMethod::Red=>vector.sort_by(|b,a| a.0[0].partial_cmp(&b.0[0]).unwrap()),
                SortMethod::Green=>vector.sort_by(|b,a| a.0[1].partial_cmp(&b.0[1]).unwrap()),
                SortMethod::Blue=>vector.sort_by(|b,a| a.0[2].partial_cmp(&b.0[2]).unwrap()),

            }
        }
    }



    fn loop_body(&mut self,
                 asdf:&mut ImageBuffer<image::Rgb<u8>, Vec<u8>>,
                 mask:&mut ImageBuffer<image::Rgb<u8>, Vec<u8>>,
                 x:u32,
                 y:u32,
                 mask_first:&mut u32,
                 mask_last:&mut u32,
                 buffer:&mut Vec<image::Rgb<u8>>,
                 maskmax:f32,
                 maskmin:f32,

    ){


        let pix=mask.get_pixel_mut(x, y);

        let lum=match self.sorting_method {
            SortMethod::Vibrance=>rgb_to_hsl(&*pix).2,
            SortMethod::Hue=>rgb_to_hsl(&*pix).0,
            SortMethod::Saturation=>rgb_to_hsl(&*pix).1,

            //a.0 = rgb array a.0[0] =red
            SortMethod::Red=>pix.0[0] as f32,
            SortMethod::Green=>pix.0[1] as f32,
            SortMethod::Blue=>pix.0[2] as f32,

        };

        //mask
        //we need to check first changed and last changed
        if lum <maskmax && lum > maskmin {
            if *mask_first==99999999{
                match self.sort_direction {
                    SortDirection::Horizontal=> *mask_first=x,
                    SortDirection::Vertical=> *mask_first=y,
                }
            }
            *pix = image::Rgb([255,255,255]);
            let pixel= asdf.get_pixel_mut(x, y);
            buffer.push(*pixel);
                match self.sort_direction {
                    SortDirection::Horizontal=> *mask_last=x,
                    SortDirection::Vertical=> *mask_last=y,
                }
        }else{
            *pix = image::Rgb([0,0,0]);
        }
    }




}



impl Default for PixelSort{

    fn default() -> Self {
        Self {
            widgetid:"".to_string(),
            max_mask:250.0,
            min_mask:100.0,
            sorting_method:SortMethod::Vibrance,
            sort_direction:SortDirection::Horizontal,
            invert_sort:false,
        }
    }
}



impl imagefilter::ImageFilter for PixelSort{


    fn apply_filter(&mut self,image: image::ImageBuffer<image::Rgb<u8>,Vec<u8>>)->image::ImageBuffer<image::Rgb<u8>,Vec<u8>> {
        
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

        let mut mask= asdf.clone();

        //loop all rows

        match self.sort_direction {
            SortDirection::Horizontal=>
            {

                for y in miny..maxy{
                    let mut buffer:Vec<image::Rgb<u8>>=vec![];

                    //first and last changed value in the mask
                    let mut mask_first:u32=99999999;
                    let mut mask_last:u32=0;

                    for x in minx..maxx{
                        self.loop_body(&mut asdf,&mut mask,x,y,&mut mask_first,&mut mask_last,&mut buffer,maskmax,maskmin);
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
            },

            SortDirection::Vertical=>
            {
                for x in minx..maxx{
                    let mut buffer:Vec<image::Rgb<u8>>=vec![];

                    //first and last changed value in the mask
                    let mut mask_first:u32=99999999;
                    let mut mask_last:u32=0;

                    for y in miny..maxy{
                        self.loop_body(&mut asdf,&mut mask,x,y,&mut mask_first,&mut mask_last,&mut buffer,maskmax,maskmin);
                    }
                    //sort
                    self.sort_pixel_vector(&mut buffer);

                    //reassign
                    let mut ind=0;
                    for y in mask_first..mask_last {
                        //mask pixel
                        let mpix=mask.get_pixel_mut(x, y);
                        if mpix.0[0]+mpix.0[1]+mpix.0[2]!=0{
                            let pixel= asdf.get_pixel_mut(x, y);
                            *pixel=buffer[ind];
                            ind+=1;
                        }
                    }

                }


            },
        }
        asdf
    }

    fn spawn_filter_widget(&mut self,ui:&mut egui::Ui) {
        ui.heading("Pixel Sorting");
        let min_selector:f32;
        let max_selector:f32;

        match self.sorting_method {
            SortMethod::Vibrance=>{min_selector=0.0;max_selector=1.0},
            SortMethod::Hue=>{min_selector=0.0;max_selector=360.0},
            SortMethod::Saturation=>{min_selector=0.0;max_selector=1.0}

            //a.0 = rgb array a.0[0] =red
            SortMethod::Red=>{min_selector=0.0;max_selector=255.0},
            SortMethod::Green=>{min_selector=0.0;max_selector=255.0},
            SortMethod::Blue=>{min_selector=0.0;max_selector=255.0},
            
        }


        ui.add(egui::Slider::new(&mut self.min_mask, min_selector..=max_selector).text("min"));
        ui.add(egui::Slider::new(&mut self.max_mask, self.min_mask..=max_selector).text("max"));
                
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

        ui.push_id(&self.widgetid, |ui|{
            egui::ComboBox::from_label(format!("sorting direction {}",self.widgetid))
               .selected_text(format!("{}",self.sort_direction))
               .show_ui(ui, |ui|{
                   ui.selectable_value(&mut self.sort_direction, SortDirection::Horizontal, "horizontal");
                   ui.selectable_value(&mut self.sort_direction, SortDirection::Vertical, "vertical");
           });
        });
        ui.add(egui::Checkbox::new(&mut self.invert_sort,"invert"));

        
    }


}



