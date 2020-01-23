extern crate cgmath;

use engine::gl as gl;
use camera::Camera;
use spritebatch::SpriteBatch;
use spritebatch::SpriteSortMode;
use shader::Shader;
use viewportadapter::ViewportAdapter;
use viewportadapter::ViewportAdapterTrait;
use scene::Scene;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use rectangle::Rectangle;

pub trait Renderer<T> {
    fn before_render(&self, scene: &Scene<T>) {

    }

    fn render_begin <'sb>(&self, camera: &'sb mut Camera<ViewportAdapter>, spritebatch: &'sb mut SpriteBatch, shader: Shader) {
        let mut viewport = Rectangle::new(0.0, 0.0, 0, 0);
        // Sets the current camera viewport if the camera has one
        match camera.get_viewport_adapter() {
            &Some(va) => {
                let r = va;
                let rr = r.get_viewport();
                let vp = Rect::new(rr.x as i32, rr.y as i32, rr.w as u32, rr.h as u32);
                unsafe {
                    gl::Viewport(vp.x, vp.y, vp.width() as i32, vp.height() as i32);
                }
            },
            _ => {}
        }

        // MonoGame resets the Viewport to the RT size without asking so we have to let the Camera know to update itself
        camera.force_matrix_update();

        let m = camera.get_transform_matrix();
        spritebatch.begin(viewport, SpriteSortMode::SpriteSortModeDeferred, Some(shader), Some(m));
    }
    
    fn render_end <'sb>(&self, scene: &Scene<T>, viewport: Rectangle, spritebatch: &'sb mut SpriteBatch);
    
    fn after_render(&self, scene: &Scene<T>) {

    }

    fn render(&self, scene: &Scene<T>) {
        //self.camera.force_matrix_update();
        //let m = self.camera.get_transform_matrix();
        //spritebatch.begin(renderer, SpriteSortMode::SpriteSortModeDeferred, Some(shader), Some(m));
        //Game::GetSpriteBatch()->Begin(
        //  SpriteBatch::SpriteSortModeDeferred, shader, &m, camera->GetViewportAdapter()->GetViewport());
    }
}