pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw + ?Sized> {
    pub components: Vec<Box<T>>,
}

impl<T> Screen<T>
where
    T: Draw + ?Sized,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        // 実際にボタンを描画するコード
    }
}
