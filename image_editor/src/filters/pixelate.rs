use crate::imagefilter;


pub struct PixelateFilter{
    radius:usize,
}

impl Default for PixelateFilter{
    fn default() -> Self {
        PixelateFilter {radius:2}
    }
}




impl imagefilter::ImageFilter for PixelateFilter{

    fn apply_filter(&mut self,image:image::ImageBuffer<image::Rgb<u8>,Vec<u8>>)->image::ImageBuffer<image::Rgb<u8>,Vec<u8>> {

        //loop all pixels offsetted bu radius
        //
        let mut img=image.clone();

        for y in (0..image.height()).step_by(self.radius){
            for x in (0..image.width()).step_by(self.radius){
                let pix = image.get_pixel(x, y);

                for offsety in 0..self.radius as u32{
                    for offsetx in 0..self.radius as u32{
                        let cx= offsetx+x;
                        let cy= offsety+y;

                        let xinbounds= cx>=0 && cx<image.width();
                        let yinbounds= cy>=0 && cy<image.height();
                        if xinbounds && yinbounds{
                            img.put_pixel(cx, cy, *pix);
                        }

                    }
                }


            }
        }

        return img;
    }


    fn spawn_filter_widget(&mut self,ui:&mut egui::Ui) {
        ui.heading("Pixelate");
        ui.add(egui::Slider::new(&mut self.radius, 2..=20).text("blur radius"));
    }

}
