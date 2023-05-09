

use crate::imagefilter;

use std::cmp::Ordering;
use std::ops::Index;

use rand::Rng;


#[derive(Eq)]
struct Piece{
    val:u8,
    index:u16,
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val) 
    }
}


impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.val==other.val
    }
}


#[derive(Debug,Clone, Copy, PartialEq)]
pub enum SortMethod{
    Hue,
    Vibrance,
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


    fn sort_pixel_vector(&self,vector:Vec<image::Rgb<u8>>)->Vec<image::Rgb<u8>>{
        let mut secvec:Vec<Piece>=vec![];

        let mut i=0;//cahgne to enumberage
        for item in vector.iter(){

            //let total = get_luminance(item);
            let total = match self.sorting_method {
                SortMethod::Vibrance=>get_luminance(item),
                SortMethod::Hue=>get_hue(item),
                
            };


            let a =Piece{val:total,index:i};
            secvec.push(a);
            i+=1;
        }


        secvec.sort(); 
        
        let mut retvec:Vec<image::Rgb<u8>>=vec![];
        for (_ind,item) in secvec.iter().enumerate(){
            retvec.push(vector.index(item.index as usize).clone());
        }


        retvec
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


//this does not get hue accurately.. at all but the results are kinda cool
fn get_hue(pixel:&image::Rgb<u8>)->u8{
    let r=pixel.0[0] / 255;
    let g=pixel.0[1] / 255;
    let b=pixel.0[2] / 255;
    let mut arra=[r,g,b];

    arra.sort();

    let mut hue:u8=0;
    //what is biggest and what is smalles
    //put in array and sort
    //
    if arra[2]-arra[0]==0 {
        return 0
    }

    if arra[0]==r {
        hue = (g-b)/(arra[2]-arra[0])
    }

    if arra[0]==g{
        hue = 2+(b-r)/(arra[2]-arra[0])
    }

    if arra[0]==b{
        hue = 4+(r-g)/(arra[2]-arra[0])
    }
    hue
}







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
            let sorted_thing=self.sort_pixel_vector(buffer);

            //reassign
            let mut ind=0;
            for x in mask_first..mask_last {
                //mask pixel
                let mpix=mask.get_pixel_mut(x, y);
                if mpix.0[0]+mpix.0[1]+mpix.0[2]!=0{
                    let pixel= asdf.get_pixel_mut(x, y);
                    *pixel=sorted_thing[ind];
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
           });
        });

            /*
            if ui.button("sort ").clicked() {
                self.sort_image();
                self.save_image();
            }
             * */
        
    }


}



