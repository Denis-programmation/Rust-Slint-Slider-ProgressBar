slint::slint!
{
    #[edition = "1.19"]
    
    import { ProgressIndicator, VerticalBox } from "std-widgets.slint";
        
    export component Fenetre inherits Window
    {
        width: 150px;
        height: 100px;
        
        in property <float> avancement: 0.0;
        
        VerticalBox
        {

            ProgressIndicator
            {
                width: 100px;
                height: 20px;
                progress <=> avancement;    // valeurs comprises entre 0.00 et 1.00
            }
        }

    }
}

//use std::thread;
use std::time::Duration;
use slint::{Timer, TimerMode};

fn main()
{
    let fenetre = Fenetre::new().unwrap();
    let fenetre2 = fenetre.clone_strong();
    let mut increment = 0;
    let timer = Timer::default();

    timer.start 
    (
        TimerMode::Repeated,
        Duration::from_secs(1),
        move || 
        {
            if increment <= 90
            {
                increment += 10;
                let f = ( increment as f32 ) / 100.0;
                fenetre2.set_avancement ( f );
            } else 
            {
//                timer.stop()
            }
        },
    );

/*   for i in (0..=100).step_by(10)
    {
        let f = ( i as f32 ) / 100.0;
        fenetre.set_avancement ( f );
        fenetre.window().request_redraw();
        thread::sleep(Duration::from_millis(1000));
    }*/
    
    fenetre.run().unwrap();
}
