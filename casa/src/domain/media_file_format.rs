#[derive(Eq, PartialEq, Debug, ToSql, FromSql)]
#[postgres(name = "account_type")]
pub enum MediaFileFormat {
    #[postgres(name = "png")]
    PNG,
    #[postgres(name = "jpeg")]
    JPEG,
    #[postgres(name = "bmp")]
    BMP,
    #[postgres(name = "gif")]
    GIF
}
