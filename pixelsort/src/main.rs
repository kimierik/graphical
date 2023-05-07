


#[derive(Eq)]
struct Piece{
    val:u8,
    index:u16,
}

use std::{cmp::Ordering, ops::{Index, IndexMut}};


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



fn get_luminance(pixel:&image::Rgb<u8>)->u8{
        let val1=(0.2126*pixel.0[0] as f32  ) as u8;
        let val2=(0.7152*pixel.0[1] as f32  ) as u8;
        let val3=(0.0722*pixel.0[2] as f32  ) as u8;
        let total = val1+val2+val3 ;
        total
}



fn sorring(vector:Vec<image::Rgb<u8>>)->Vec<image::Rgb<u8>>{
    let mut secvec:Vec<Piece>=vec![];


    let mut i=0;//cahgne to enumberage
    for item in vector.iter(){

        //let total=item.0[0]+item.0[1]+item.0[2];
        let val1=(0.2126*item.0[0] as f32  ) as u8;
        let val2=(0.7152*item.0[1] as f32  ) as u8;
        let val3=(0.0722*item.0[2] as f32  ) as u8;
        let total = val1+val2+val3 ;


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









fn main() {
    let img = image::open("./images/sakura_samurai.png").unwrap();


    let mut asdf=img.clone().into_rgb8();
    
    let width= asdf.width();
    let height= asdf.height();

    //area definition
    //if we want to limit sort to a certain area of the screen
    let minx=0;
    let maxx=width;
    let miny=0;
    let maxy=height;


    let maskmin=100;
    let maskmax=250;

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
        let sorted_thing=sorring(buffer);

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





    match asdf.save("./images/new.png")  {
        Ok(_a)=>(),
        Err(e)=> println!("{}",e)
    }
}
