
use std::{cmp::Ordering, ops::{Index, IndexMut}, path::Display};

use image::ImageBuffer;



#[derive(Debug,Clone, Copy, PartialEq)]
enum SortMethod{
    Vibrance,
    Hue,
}



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


struct SortParams{
    pub max_mask:u8,
    pub min_mask:u8,
    pub sorting_method:SortMethod,
}

impl Default for SortParams {
    fn default() -> Self {
        Self { 
            max_mask: 250,
            min_mask: 100 ,
            sorting_method:SortMethod::Vibrance,
        }
    }
}


impl std::fmt::Display for SortMethod {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"{:?}",self)
    }
    
}



struct AppWidget{
    filename:String,
    image:ImageBuffer<image::Rgb<u8>,Vec<u8>>,
    pub sort_params:SortParams,
}


impl Default for AppWidget{
    fn default() -> Self {
        let img = image::open("./images/sakura_samurai.png").unwrap();
        let asdf=img.clone().into_rgb8();
        Self { 
            filename: "./images/sakura_samurai.png".to_string(),
            image:asdf ,
            sort_params:SortParams::default() 
        }
    }
}



impl AppWidget {
    
    pub fn save_image(&self){
        match self.image.save("./images/new.png")  {
            Ok(_a)=>(),
            Err(e)=> println!("{}",e)
        }
    }

    pub fn sort_image(&mut self){

        let img = image::open(&self.filename).unwrap();
        let mut asdf=img.clone().into_rgb8();
        
        let width= asdf.width();
        let height= asdf.height();

        //area definition
        //if we want to limit sort to a certain area of the screen
        let minx=0;
        let maxx=width;
        let miny=0;
        let maxy=height;

        let maskmin=self.sort_params.min_mask;
        let maskmax=self.sort_params.max_mask;

        //could possiubly be clone of asdf not just another intorgb
        let mut mask= img.clone().into_rgb8();

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
                    mask_last=x;
                }else{
                    *pix = image::Rgb([0,0,0]);
                }

                //prob can be removed and moved to the if inside lum thing
                if pix.0[0]+pix.0[1]+pix.0[2]!=0{
                    let pixel= asdf.get_pixel_mut(x, y);
                    buffer.push(*pixel);
                }
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
        self.image=asdf;
    }

    fn sort_pixel_vector(&self,vector:Vec<image::Rgb<u8>>)->Vec<image::Rgb<u8>>{
        let mut secvec:Vec<Piece>=vec![];

        let mut i=0;//cahgne to enumberage
        for item in vector.iter(){

            //let total = get_luminance(item);
            let total = match self.sort_params.sorting_method {
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





impl eframe::App for AppWidget{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("image pixel sorter");

            ui.horizontal(|ui| {
                let name_label = ui.label("FileName: ");
                ui.text_edit_singleline(&mut self.filename)
                    .labelled_by(name_label.id);
            });

            ui.heading("Pixel Sorting");
            ui.add(egui::Slider::new(&mut self.sort_params.min_mask, 0..=255).text("min"));
            ui.add(egui::Slider::new(&mut self.sort_params.max_mask, self.sort_params.min_mask..=255).text("max"));
            
            egui::ComboBox::from_label("sorting method")
                   .selected_text(format!("{}",self.sort_params.sorting_method))
                   .show_ui(ui, |ui|{
                       ui.selectable_value(&mut self.sort_params.sorting_method, SortMethod::Vibrance, "vib");
                       ui.selectable_value(&mut self.sort_params.sorting_method, SortMethod::Hue, "hue");
                   });

            if ui.button("sort ").clicked() {
                self.sort_image();
                self.save_image();
            }




            //ui.label(format!("Hello '{}', age {}", self.name, self.age));
            if ui.button("close").clicked(){
                frame.close();
                //run game setup thing with arguments 
            }
        });

        
    }
}



use eframe::egui;

fn main() {

    tracing_subscriber::fmt::init();
    let options =eframe::NativeOptions{
        initial_window_size:Some(egui::Vec2 { x: 200.0, y: 200.0 }),
        ..Default::default()
    };

    eframe::run_native("app", options,
        Box::new(|_cc| Box::new(AppWidget::default())));
    println!( "asd");
    


}
