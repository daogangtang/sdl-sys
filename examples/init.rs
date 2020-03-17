use sdl_sys::*;

fn main() {
    println!("Initializing SDL.");

    unsafe {
	if SDL_Init(SDL_INIT_VIDEO|SDL_INIT_AUDIO) == -1 {
	    println!("Could not initialize SDL: {:?}",
		     std::ffi::CStr::from_ptr(SDL_GetError()));
	    return;
	}

	println!("SDL initialized.");

	let screen = SDL_SetVideoMode(640, 480, 32, SDL_SWSURFACE);
	SDL_Flip(screen);

	SDL_Delay(2000);

	println!("Quitting SDL.");

	SDL_Quit();
    }

    println!("Quited.");
}
