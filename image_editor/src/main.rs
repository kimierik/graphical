
pub mod imagefilter;

pub mod filters;
pub mod utils;


use filters::PixelSort;
use filters::mask::Mask;



struct EditorWidget{
    filename:String,
    filterlist:Vec<Box<dyn imagefilter::ImageFilter>>,
    widget_remove_queue:Vec<usize>,
    hovered_filter:Filters,
}


impl Default for EditorWidget {
    fn default() -> Self {
        Self { 
            filename: "./images/sakura_samurai.png".to_string(),
            filterlist: vec![Box::new(PixelSort::default())] ,
            widget_remove_queue:vec![],
            hovered_filter:Filters::PixelSort
        }
    }
    
}

impl EditorWidget{ }


#[derive(Debug, PartialEq)]
enum Filters{
    PixelSort,
    Mask,
}




impl eframe::App for EditorWidget{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("image pixel sorter");

            ui.horizontal(|ui| {
                let name_label = ui.label("FileName: ");
                ui.text_edit_singleline(&mut self.filename)
                    .labelled_by(name_label.id);
            });

            //adding filters
            egui::ComboBox::from_label("filter selection")
                   .selected_text(format!("{:?}",self.hovered_filter))
                   .show_ui(ui, |ui|{
                       ui.selectable_value(&mut self.hovered_filter, Filters::PixelSort, "pixelsort");
                       ui.selectable_value(&mut self.hovered_filter, Filters::Mask, "mask");
                   });
                if ui.button("add filter").clicked(){
                    self.filterlist.push(match self.hovered_filter {
                        Filters::PixelSort=>Box::new(PixelSort::make_with_id(rand::random::<u32>().to_string())),
                        Filters::Mask=>Box::new(Mask::make_with_id(rand::random::<u32>().to_string())),
                    })
                }


            ui.heading("filters");


            //remove widgets
            for item in &self.widget_remove_queue{
                self.filterlist.remove(*item);
            }
            self.widget_remove_queue=vec![];

            let mut index=0;
            for i in &mut self.filterlist{
                //widgets are removed by adding them to a remove queue they are deleted next cycle
                //this is the way it is because of how rust handles mutation
                if ui.button("remove filter").clicked(){
                    self.widget_remove_queue.push(index);
                }
                i.spawn_filter_widget(ui);
                index+=1;
            }


            if ui.button("apply filters ").clicked() {
                let img = image::open(&self.filename).unwrap();
                let mut image=img.clone().into_rgb8();

                for i in &mut self.filterlist{
                    image=i.apply_filter(image);
                }
                //move this to function
                match image.save("./images/new.png")  {
                    Ok(_a)=>(),
                    Err(e)=> println!("{}",e)
                }
            }


            if ui.button("close").clicked(){
                frame.close();
            }
        });

        
    }
}



fn main() {
    tracing_subscriber::fmt::init();
    let options =eframe::NativeOptions{
        initial_window_size:Some(egui::Vec2 { x: 200.0, y: 200.0 }),
        ..Default::default()
    };

    eframe::run_native("app", options,
        Box::new(|_cc| Box::new(EditorWidget::default()))).unwrap();
}
