


use std::{vec, print, println};

use crate::imagefilter::ImageFilter;

pub struct Blur{ }



impl ImageFilter for Blur{
    fn apply_filter(&mut self,image:image::ImageBuffer<image::Rgb<u8>,Vec<u8>>)->image::ImageBuffer<image::Rgb<u8>,Vec<u8>> {
        let mut newimage= image.clone();
        

        //area definition
        //if we want to limit sort to a certain area of the screen
        let minx=0;
        let maxx=image.width();
        let miny=0;
        let maxy=image.height();

        let radius:i32=1;

        for y in miny..maxy{
            for x in minx..maxx{
                //get surroinding pixel
                //eventually we should be able to say get aver pixel at radius but lets not do
                //that now
                //get average r,g&b . that is the rgb values for newimage pixel at x,y

                let ofs=self.get_offsets(x,y,radius,&image);
                //get average r,g,b from the vector of pixels
                if ofs.len()>0{
                    let pix=newimage.get_pixel_mut(x, y);
                    *pix=self.get_average(&ofs);
                }else{
                    println!("offsets is 0 len");
                }
            }
        }


        
        newimage
    }

    fn spawn_filter_widget(&mut self,ui:&mut egui::Ui) {
        ui.heading("mask");
        ui.heading("new options coming soon");
        
    }

}



impl Blur{

    pub fn get_offsets(&self,x:u32,y:u32,radius:i32,img :&image::ImageBuffer<image::Rgb<u8>,Vec<u8>> )->Vec<image::Rgb<u8>>{


        let mut offsets:Vec<image::Rgb<u8>>=vec![];

        for offset_y in -radius..radius{

            let comp_y= y+offset_y as u32;

            for offset_x in -radius..radius{
                let comp_x= x+offset_x as u32 ;

                if comp_x>0 && comp_x< img.width() {
                    if comp_y>0 && comp_y< img.height() {
                        offsets.push( *img.get_pixel(comp_x,comp_y));

                    }
                }
            }
        }

        offsets
    }

    pub fn get_average(&self,pixels:& Vec<image::Rgb<u8>>)->image::Rgb<u8>{
        //average is the average of all the values
        //add all reds together and div it with the size of the vec
        //do same with the rest
        //is there some cool one liner we could do
        let mut r=0;
        let mut g=0;
        let mut b=0;
        for pix in pixels{
            r+= pix.0[0] as u32;
            g+= pix.0[1] as u32;
            b+= pix.0[2] as u32;
        }
        r/=pixels.len() as u32;
        g/=pixels.len() as u32;
        b/=pixels.len() as u32;


        image::Rgb([r as u8,g as u8,b as u8])

    }



}


