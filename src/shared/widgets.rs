use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    FadeView = <CachedView> {
        draw_bg: {
            instance opacity: 1.0
            
            fn pixel(self) -> vec4 {
                let color = sample2d_rt(self.image, self.pos * self.scale + self.shift) + vec4(self.marked, 0.0, 0.0, 0.0);
                return Pal::premul(vec4(color.xyz, color.w * self.opacity))
            }
        }
    }

    CenteredScaledImage = <Image> {
        draw_bg: {
            texture image: texture2d
            instance opacity: 1.0
            instance image_scale: vec2(1.0, 1.0)

            fn get_color_scale(self, scale: vec2) -> vec4 {
                let traslation = vec2(0.0, 0.0);
                if scale.x > 1.0 {
                    traslation.x = 0.5 - 1.0 / (scale.x * 2.0);
                }
                if scale.y > 1.0 {
                    traslation.y = 0.5 - 1.0 / (scale.y * 2.0);
                }
                return sample2d(self.image, (self.pos - traslation) * scale).xyzw;
            }

            fn get_color(self) -> vec4 {
                return self.get_color_scale(self.image_scale)
            }

            fn pixel(self) -> vec4 {
                let color = self.get_color();
                return Pal::premul(vec4(color.xyz, color.w * self.opacity))
            }
        }
    }

    CenteredOnTop = <Image> {
        draw_bg: {
            instance radius: 90.
            instance opacity: 1.0
            instance image_scale: vec2(1.0, 1.0)

            fn get_opacity(self) -> float {
                return self.opacity;
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1,
                    1,
                    self.rect_size.x - 2.0,
                    // This calculation is to make sure the bottom part is not rounded
                    self.rect_size.y + self.radius * 2.0,
                    max(1.0, self.radius)
                );

                let color = self.get_color();
                sdf.fill_keep(Pal::premul(vec4(color.xyz, color.w * self.get_opacity())));
                return sdf.result
            }
        }
    }

    CenteredOnBottom = <Image> {
        draw_bg: {
            instance radius: 60.
            instance opacity: 1.0
            instance image_scale: vec2(1.0, 1.0)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box_y(
                    1,
                    1,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    1.0,
                    max(1.0, self.radius)
                );

                let color = self.get_color();
                sdf.fill_keep(Pal::premul(vec4(color.xyz, color.w * self.opacity)));
                return sdf.result
            }
        }
    }

    Line = <View> {
        width: Fill,
        height: 1,
        show_bg: true,
        draw_bg: {
            color: #8b9e77
        }
    }

    VerticalLine = <View> {
        width: 1,
        height: Fill,
        show_bg: true,
        draw_bg: {
            color: #8b9e77
        }
    }

    IconButton = <Button> {
        draw_icon: {
            fn get_color(self) -> vec4 {
                return #fff
            }
        }
        icon_walk: {width: 7.5, height: Fit}
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                return sdf.result
            }
        }
        padding: 9.0
        text: ""
    }

    VerticalFadingBar = <RoundedYView> {
        width: 10.0
        height: 50.0

        show_bg: true
        draw_bg: {
            color: #fff
            instance opacity: 0.3

            fn get_color(self) -> vec4 {
                return mix(
                    vec4(self.color.rgb, 0.0),
                    vec4(self.color.rgb, self.opacity),
                    self.pos.y);
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box_y(
                    self.inset.x + self.border_width,
                    self.inset.y + self.border_width,
                    self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                    self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                    self.radius.x,
                    self.radius.y
                )
                sdf.fill_keep(self.get_color())
                if self.border_width > 0.0 {
                    sdf.stroke(self.get_border_color(), self.border_width)
                }
                return sdf.result;
            }
        }
    }
}