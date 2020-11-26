//! **WARNING**: Total WIP, don't use!

// #![doc(hidden)]

pub use crate::{
    camera, collections, coroutines, file, input, material, models,
    shapes, time, window, main,
    types::Rect, // TODO: find a module?

    texture,

    // TODO: texture
};

pub use glam as math;
pub use quad_rand as rand;
pub use miniquad::date;

#[cfg(feature = "log-impl")]
pub use crate::logging;

pub mod prelude {
    //! Mose common types that can be glob-imported `use macroquad::experimental::prelude::*`
    //! for convenience.

    pub use crate::experimental::{
        camera::*,
        file::*,
        input::*,
        material::*,
        models::*,
        shapes::*,
        texture::*,
        time::*,
        window::*,
        rand::*,
        graphics::{self, colors::*, Color},
        math::*,
        Rect,
        collections,
        coroutines,
    };

    pub use glam::{self, *};
    pub use miniquad::{conf::Conf, Comparison, PipelineParams, UniformType};
    // pub use quad_gl::{colors::*, Color, GlPipeline, QuadGl, Vertex, DrawMode}; // TODO: ?

    #[cfg(feature = "log-impl")]
    pub use crate::experimental::logging::*;
}

// TODO: re-export Color and colors?
pub mod graphics {
    pub use quad_gl::{colors, Color};
    pub use crate::experimental::{
        text::{Font, Text},
        texture::Texture2D,
        shapes2::{Rectangle, Line},
    };
}

mod texture2 {
    use crate::{texture as internal, get_context, file::load_file};

    pub struct Texture2D(pub(in crate::experimental) internal::Texture2D);

    impl Texture2D {
        /// Load texture from file into GPU memory
        pub async fn load(path: &str) -> Self {
            let bytes = load_file(path)
                .await
                .unwrap_or_else(|e| panic!("Error loading texture: {}", e));
            let context = &mut get_context().quad_context;

            Self(internal::Texture2D::from_file_with_format(context, &bytes[..], None))
        }
    }

    // TODO: call delete_texture in Drop!

    // TODO: ?
    // pub fn draw_texture(texture: Texture2D, x: f32, y: f32, color: Color) {
    //     draw_texture_ex(texture, x, y, color, Default::default());
    // }

    // TODO: ?
    // pub fn draw_texture_ex(
    //     texture: Texture2D,
    //     x: f32,
    //     y: f32,
    //     color: Color,
    //     params: DrawTextureParams,
    // ) {

    // pub struct DrawTextureParams {
    //     pub dest_size: Option<Vec2>,
    //
    //     /// Part of texture to draw. If None - draw the whole texture.
    //     /// Good use example: drawing an image from texture atlas.
    //     /// Is None by default
    //     pub source: Option<Rect>,
    //
    //     /// Rotation in radians
    //     pub rotation: f32,
    //
    //     /// Rotation around this point
    //     pub pivot: Option<Vec2>,
    // }
}

mod shapes2 {
    use crate::{
        shapes as internal,
        texture,
        experimental::{
            graphics::{self, colors, Color, Texture2D},
            // Rect,
            math::Vec2,
        },
    };

    // pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
    pub struct Line {
        pub from: Vec2,
        pub to: Vec2,

        // TODO: consider extracting these fields to a reusable (Rectangle?) struct like LineProperties
        pub thickness: f32,
        pub color: Color,
    }

    impl Line {
        pub fn new(from: Vec2, to: Vec2, thickness: f32, color: Color) -> Self {
            Self { from, to, thickness, color}
        }

        pub fn draw(&self) {
            internal::draw_line(self.from.x(), self.from.y(), self.to.x(), self.to.y(), self.thickness, self.color)
        }
    }

    #[allow(dead_code)] // TODO: !
    pub struct Rectangle {
        pub texture: Option<graphics::Texture2D>,

        pub color: Option<Color>,

        // TODO: extract this to a new reusable (Line?) struct like LineProperties?
        pub border_color: Option<Color>,
        pub border_thickness: Option<f32>,

        pub dest_size: Option<Vec2>,

        // TODO:
        // /// Part of texture to draw. If None - draw the whole texture.
        // /// Good use example: drawing an image from texture atlas.
        // /// Is None by default
        // pub source: Option<Rect>,

        // TODO:
        // /// Rotation in radians
        // pub rotation: f32,
        //
        // /// Rotation around this point
        // pub pivot: Option<Vec2>,

        // TODO: put fields from DrawTextureParams here
    }

    impl Rectangle {
        // pub fn ???
        pub fn from_texture(texture: Texture2D) -> Self {
            Self {
                texture: Some(texture),
                dest_size: None,
                color: None,
                border_color: None,
                border_thickness: None,
            }
        }

        // pub fn from_

        // TODO: add constructors for non-textured variants
        // Drawable::SolidRect { rect } => {
        //     shapes::draw_rectangle(self.pos.x(), self.pos.y(), rect.w, rect.h, self.color);
        // }

        // Drawable::LinesRect { rect, thickness } => {
        //     shapes::draw_rectangle_lines(
        //         self.pos.x(),
        //         self.pos.y(),
        //         rect.w,
        //         rect.h,
        //         *thickness,
        //         self.color,
        //     );
        // }

        pub fn with_dest_size(mut self, value: Vec2) -> Self {
            self.dest_size = Some(value);
            self
        }

        pub fn draw_at(&self, pos: Vec2) {
            if let Some(ref texture) = self.texture {
                let color = self.color.unwrap_or(colors::WHITE);
                let params = texture::DrawTextureParams {
                    dest_size: self.dest_size,
                    ..Default::default()
                };
                texture::draw_texture_ex(texture.0, pos.x(), pos.y(), color, params);
            } else {
                todo!()
            }
        }
    }
}

mod text {
    use std::borrow::Cow;

    use crate::{text as internal, experimental::graphics::{Color, colors, Texture2D}};

    /// TTF font loaded to GPU
    #[derive(Clone, Copy, Debug, PartialEq, Default)]
    pub struct Font(internal::Font);

    impl Font {
        pub async fn load(path: &str) -> Self {
            Self(internal::load_ttf_font(path).await)
        }

        pub fn load_from_bytes(bytes: &[u8]) -> Self {
            Self(internal::load_ttf_font_from_bytes(bytes))
        }

        pub fn default() -> Self {
            Self(internal::Font::default())
        }

        /// List of ascii characters, may be helpfull in combination with "populate_font_cache"
        pub fn ascii_character_list() -> Vec<char> {
            internal::Font::ascii_character_list()
        }

        pub fn populate_font_cache(&self, characters: &[char], size: u16) {
            self.0.populate_font_cache(characters, size)
        }

        pub fn texture(&self) -> Texture2D {
            Texture2D(self.0.texture())
        }
    }

    #[derive(Debug, Clone)]
    pub struct Text<'a> {
        pub label: Cow<'a, str>,

        pub font: Font,

        /// Base size for character height. The size in pixel used during font rasterizing.
        pub font_size: u16,

        /// The glyphs sizes actually drawn on the screen will be font_size * font_scale
        /// However with font_scale too different from 1.0 letters will be blurry
        pub font_scale: f32,

        pub color: Color,
    }

    impl<'a> Text<'a> {
        pub fn new<S: Into<Cow<'a, str>>>(label: S) -> Self {
            Self {
                label: label.into(),
                font: Font::default(),
                font_size: 20,
                font_scale: 1.0,
                color: colors::WHITE,
            }
        }

        pub fn with_font(mut self, value: Font) -> Self {
            self.font = value;
            self
        }

        pub fn with_font_size(mut self, value: u16) -> Self {
            self.font_size = value;
            self
        }

        pub fn with_font_scale(mut self, value: f32) -> Self {
            self.font_scale = value;
            self
        }

        pub fn with_color(mut self, value: Color) -> Self {
            self.color = value;
            self
        }

        pub fn measure_text(&self) -> (f32, f32) {
            internal::measure_text(&self.label, Some(self.font.0), self.font_size, self.font_scale)
        }

        pub fn draw_at(&self, x: f32, y: f32) {
            let params = internal::TextParams {
                font: self.font.0,
                font_size: self.font_size,
                font_scale: self.font_scale,
                color: self.color,
            };
            internal::draw_text_ex(self.label.as_ref(), x, y, params)
        }
    }
}
