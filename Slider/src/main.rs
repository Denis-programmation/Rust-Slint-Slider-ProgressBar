slint::slint!
{
    #[edition = "1.19"]
    
    import { VerticalBox, Slider } from "std-widgets.slint";
        
    export component Fenetre inherits Window
    {
        width: 400px;
        height: 300px;
        
        out property <float> valeur: 0.0;

        callback envoi_valeur(float);
        
        VerticalBox
        {
            padding-bottom:10px;
            
            Slider
            {
                value <=> root.valeur;
                changed =>
                {
                    root.envoi_valeur(root.valeur);
                }
            }
        }
    }
}

fn main()
{
    let fenetre = Fenetre::new().unwrap();

    fenetre.on_envoi_valeur ( | f | { println!( "Valeur du curseur : {f}" ) } );
    
    fenetre.run().unwrap();
}
