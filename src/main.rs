// use embedded_svc::anyerror::*;

#[allow(unused_imports)]
use esp_idf_hal::prelude::*;
#[allow(unused_imports)]
use esp_idf_svc::sysloop::*;

use std::{thread, time::Duration};

use ili9341::DisplayError;
use embedded_graphics::{mono_font::{ascii::FONT_10X20, MonoTextStyle}, pixelcolor::Rgb565, prelude::*, primitives::{
        PrimitiveStyleBuilder, Rectangle, StrokeAlignment,
    }, text::{Alignment, Text}};



fn increment(current: i8) -> i8 {
    current.wrapping_add(1)
}


fn draw_sample<T>(lcd: &mut T) 
    where T : DrawTarget<Color=Rgb565, Error=DisplayError>
{
    
    // Create a border style
    let border_stroke = PrimitiveStyleBuilder::new()
            .stroke_color(embedded_graphics::pixelcolor::Rgb565::BLUE)
            .stroke_width(8)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();

    // Draw border around the screen
    let _ = lcd
            .bounding_box()
            .into_styled(border_stroke)
            .draw(lcd);


    // Create text style
    let character_style = MonoTextStyle::new(
            &FONT_10X20,
            embedded_graphics::pixelcolor::Rgb565::BLACK);
    let text = "Rust and M5Stack !!";
    
    // Draw text
    let textdrawable = Text::with_alignment(
                text,
                lcd.bounding_box().center() + Point::new(0, 15),
                character_style,
                Alignment::Center,
            );
    let _ = textdrawable.draw(lcd);
}


fn draw_single_btn_status<T>(lcd: &mut T, label:&str, status:bool, point: Point)
    // where T: DrawTarget<Color=Rgb565, Error=DisplayError>
    where T: DrawTarget<Color=Rgb565>
{
    let character_style_normal = MonoTextStyle::new(
        &FONT_10X20,
        embedded_graphics::pixelcolor::Rgb565::new(96, 96, 96)
    );
    let character_style_pressed = MonoTextStyle::new(
        &FONT_10X20,
        embedded_graphics::pixelcolor::Rgb565::YELLOW
    );

    let size = Size::new(64, 24);
    let topleft = point;
    let center = topleft + (size / 2) + Point::new(0, 4);

    let chstyle : &MonoTextStyle<Rgb565> = if status { &character_style_pressed } else { &character_style_normal };
    let bgcolor = if status { Rgb565::RED } else { Rgb565::BLACK };

    let _ = lcd.fill_solid(&mut Rectangle::new(topleft, size), bgcolor);

    let textdrawable = Text::with_alignment(
            label,
            center,
            *chstyle,
            Alignment::Center
    );
    let _ = textdrawable.draw(lcd);
}

fn draw_btn_status<T>(lcd: &mut T, btn_a:bool,btn_b:bool,btn_c:bool)
    // where T: DrawTarget<Color=Rgb565, Error=DisplayError>
    where T: DrawTarget<Color=Rgb565>
{
    let pos2 = Point::new(320 / 2 - 32, 240 - 32);
    let pos1 = pos2 - Point::new(64 + 32, 0);
    let pos3 = pos2 + Point::new(64 + 32, 0);

    draw_single_btn_status(lcd, "Btn A", btn_a, pos1);
    draw_single_btn_status(lcd, "Btn B", btn_b, pos2);
    draw_single_btn_status(lcd, "Btn C", btn_c, pos3);
}


fn main() -> anyhow::Result<()> {

    println!("Initializing...");

    let peripherals : esp_idf_hal::peripherals::Peripherals = esp_idf_hal::peripherals::Peripherals::take().unwrap();
    let gpios : esp_idf_hal::gpio::Pins = peripherals.pins;
    let pin_btn_a = gpios.gpio39.into_input().expect("BtnA Failed.");
    let pin_btn_b = gpios.gpio38.into_input().expect("BtnB Failed.");
    let pin_btn_c = gpios.gpio37.into_input().expect("BtnC Failed.");

    let mut pin_lcd_blk = gpios.gpio32.into_output().unwrap();
    pin_lcd_blk.set_high().unwrap();
    let pin_sclk  = gpios.gpio18.into_output().unwrap();
    let pin_sdo  = gpios.gpio23.into_output().unwrap();
    let pin_cs = gpios.gpio14.into_output().unwrap();
    let pin_dc = gpios.gpio27.into_output().unwrap();
    let mut lcd_reset_pin : esp_idf_hal::gpio::Gpio33<esp_idf_hal::gpio::Output> = gpios.gpio33.into_output().unwrap();

    println!("Issue LCD Reset by GPIO pin");
    lcd_reset_pin.set_low().unwrap();
    thread::sleep(Duration::from_millis(100));
    lcd_reset_pin.set_high().unwrap();
    thread::sleep(Duration::from_millis(2000));

    println!("SPI Master");

    let lcd_spi_master = esp_idf_hal::spi::Master::<
                esp_idf_hal::spi::SPI2,
                esp_idf_hal::gpio::Gpio18<esp_idf_hal::gpio::Output>,
                esp_idf_hal::gpio::Gpio23<esp_idf_hal::gpio::Output>,
                esp_idf_hal::gpio::Gpio0<esp_idf_hal::gpio::Output>,
                esp_idf_hal::gpio::Gpio14<esp_idf_hal::gpio::Output>
            >::new(
        peripherals.spi2 as esp_idf_hal::spi::SPI2,
        esp_idf_hal::spi::Pins {
                    sclk: (pin_sclk),
                    sdo: (pin_sdo),
                    sdi: None,
                    cs: Some(pin_cs) },
        esp_idf_hal::spi::config::Config { 
            baudrate: (esp_idf_hal::units::Hertz(10 * 1000 * 1000)),
            data_mode: (esp_idf_hal::spi::config::MODE_0),
            bit_order: (esp_idf_hal::spi::config::BitOrder::MSBFirst) }
    ).unwrap();

    println!("SPI Display interface");

    let spidisplayinterface = display_interface_spi::SPIInterfaceNoCS::new(
        lcd_spi_master,
        pin_dc
    );

    println!("ILI9341");

    let mut lcd = ili9341::Ili9341::new(
        spidisplayinterface,
        lcd_reset_pin,
        &mut esp_idf_hal::delay::FreeRtos,
        ili9341::Orientation::Landscape,
        ili9341::DisplaySize240x320
    ).expect("Failed to initialize LCD ILI9341.");


    println!("ILI9341 display: {}x{}", lcd.width(), lcd.height());


    println!("Custom configuration");
    lcd.command(ili9341::Command::DisplayInvertionOn, &[ ]).expect("Failed to issue Display Invertion ON command");
    lcd.command(ili9341::Command::MemoryAccessControl, &[ 0x00 | 0x08 ]).expect("Failed to issue MemoryAccessControl command");
    let _ = lcd.fill_solid(
        &mut Rectangle::new(Point::new(0, 0), 
        Size::new(320, 240)), 
        embedded_graphics::pixelcolor::Rgb565::new(0,255,255)
    );
    
    draw_sample(&mut lcd);
    draw_btn_status(&mut lcd, false, false, false);

    let mut counter: i8 = 0;

    let mut prev_ticks:u32 = 0;

    let mut prev_btn_a = false;
    let mut prev_btn_b = false;
    let mut prev_btn_c = false;

    let mut btnstatusupdate_timer_sec:f32=0.0f32;
    let mut serialout_timer_sec : f32 = 0.0f32;


    loop {
        let now_ticks:esp_idf_sys::TickType_t;
        unsafe {
            now_ticks = esp_idf_sys::xTaskGetTickCount();
        }
        let delta_ticks:u32 = now_ticks - prev_ticks;
        unsafe {
            prev_ticks = esp_idf_sys::xTaskGetTickCount();
        }
        let delta_sec: f32 = (delta_ticks as f32) * (1.0f32 / (esp_idf_sys::configTICK_RATE_HZ as f32));

        let btn_a = pin_btn_a.is_low().unwrap();
        let btn_b = pin_btn_b.is_low().unwrap();
        let btn_c = pin_btn_c.is_low().unwrap();

        btnstatusupdate_timer_sec += delta_sec;

        if btnstatusupdate_timer_sec > 0.025f32 {
            btnstatusupdate_timer_sec = 0.0f32;

            if btn_a != prev_btn_a || btn_b != prev_btn_b || btn_c != prev_btn_c {
                draw_btn_status(&mut lcd, btn_a, btn_b, btn_c);
                prev_btn_a = btn_a;
                prev_btn_b = btn_b;
                prev_btn_c = btn_c;
            }
        }

        serialout_timer_sec += delta_sec;

        if serialout_timer_sec > 1.0 {
            serialout_timer_sec = 0.0f32;

            if btn_a {
                counter = counter.wrapping_add(10);
            }
            if btn_c {
                counter = counter.wrapping_sub(16);
            }
            
            if !btn_a {
                // BtnA not pressed.
                println!("Hello world counter={}", counter);
            } else {
                // BtnA pressed.
                println!("BtnA Pressed !! counter={}", counter);
            }

            counter = increment(counter);
        }

        thread::sleep(Duration::from_millis(10));
    }
}
