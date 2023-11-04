use std::{thread,
    time,
    time::SystemTime
};

use crate::canon::Canon;


pub struct GameLoop {

}

impl GameLoop {
    pub fn startLoop(game:&mut Canon){
        let mfps:u32 = 60;
        let mut vida_loop = true;
        let tiempo = SystemTime::now();
        let mut t_actual:u128 = 0;
        let mut t_anterior:u128 = 0;
        let mut t_espera:i128 = 0;
        let mut contador_fps:u64 = 0;
        let mut t_ultimo_segundo:u128 = 0;

        if let Ok(t) = tiempo.elapsed() {
            t_anterior = t_actual;
            t_actual = t.as_millis();
            t_ultimo_segundo = t_actual;
        }

        while vida_loop {
            contador_fps+=1;
            vida_loop = game.update((t_actual - t_anterior) as f32 / 1000.0);
            game.paint();
                
            if let Ok(t) = tiempo.elapsed() {
                t_anterior = t_actual;
                t_actual = t.as_millis();
            }

            t_espera = (1000/mfps) as i128 - (t_actual - t_anterior) as i128 + if t_espera>0 {t_espera} else { 0 };

            if t_espera > 0 {
                thread::sleep(time::Duration::from_millis(t_espera as u64))
            }
            if let Ok(t) = tiempo.elapsed() {
                //println!("t: {}, tuf: {}", t.as_millis(), t_ultimo_segundo);
                if t.as_millis()-t_ultimo_segundo>1000 {
                    contador_fps = 0;
                    t_ultimo_segundo = t.as_millis();
                }
            }
        }
    }
}