#[derive(Clone, Debug)]
pub struct ColorPicker {
    index: i32,
    colors: [String; 7],
}

impl ColorPicker {
    pub fn new() -> ColorPicker {
        ColorPicker {
            index: -1,
            colors: [
                String::from("#06A77D"),
                String::from("#C62E65"),
                String::from("#005377"),
                String::from("#D5C67A"),
                String::from("#F1A208"),
                String::from("#D36135"),
                String::from("#D63AF9"),
            ],
        }
    }

    pub fn get(&mut self) -> String {
        if self.index >= self.colors.len() as i32 - 1 {
            self.index = 0;
        } else {
            self.index = self.index + 1;
        }

        self.colors[self.index as usize].clone()
    }
}
