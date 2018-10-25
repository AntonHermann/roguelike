impl fmt::Display for Stage1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use termion::color;

        for di in self.level.iter_display() {
            match di {
                DisplayItem::Player => {
                    write!(f, "@")?
                },
                DisplayItem::Tile(t) => write!(f, "{}", t)?,
                DisplayItem::LineBreak => write!(f, "\n\r")?,
            }
        }
        Ok(())
    }
}