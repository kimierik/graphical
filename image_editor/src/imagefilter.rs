
pub trait ImageFilter{

    fn apply_filter(&mut self,image:image::ImageBuffer<image::Rgb<u8>,Vec<u8>>)->image::ImageBuffer<image::Rgb<u8>,Vec<u8>> ;
    
    fn spawn_filter_widget(&mut self,ui:&mut egui::Ui);
}




