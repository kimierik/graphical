
pub trait ImageFilter{

    /// applies filter to given image
    /// returns filtered image
    fn apply_filter(&mut self,image:image::ImageBuffer<image::Rgb<u8>,Vec<u8>>)->image::ImageBuffer<image::Rgb<u8>,Vec<u8>> ;

    /// the ui for the image filter goes here
    fn spawn_filter_widget(&mut self,ui:&mut egui::Ui);
}




