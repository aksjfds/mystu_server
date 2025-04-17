/* --------------------------------- // 本机ip -------------------------------- */
pub fn my_ip() -> std::net::SocketAddr {
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    socket.connect("8.8.8.8:80").unwrap();

    socket.local_addr().unwrap()
}

/* ------------------------ // 随机8位字符串验证码 12345678 ----------------------- */
pub fn random_verify_code() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::rng();

    (0..8)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/* --------------------------------- // 邮件发送 -------------------------------- */
pub fn stu<T>(to_email: &str, content: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: lettre::message::IntoBody,
{
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    // 配置邮件服务器
    let smtp_credentials = Credentials::new(
        String::from("22qyli13@stu.edu.cn"), // 你的邮箱地址
        String::from("xp289325"),            // 你的邮箱密码或应用专用密码
    );

    // 使用 STARTTLS 明确配置
    let mailer = SmtpTransport::starttls_relay("smtp.partner.outlook.cn")?
        .port(587)
        .credentials(smtp_credentials)
        .build();

    // 配置邮件内容
    let email = Message::builder()
        .from("AKSJFDS <22qyli13@stu.edu.cn>".parse()?)
        .to(to_email.parse()?)
        .subject("MyStu 验证码")
        .header(ContentType::TEXT_PLAIN)
        .body(content)?;

    // 发送邮件
    let result = mailer.send(&email);
    match result {
        Ok(_) => println!("邮件发送成功"),
        Err(e) => eprintln!("邮件发送失败: {:?}", e),
    }

    Ok(())
}

// jgsqxzdvwumeebfa
// rpmmbdjxathyebic
pub fn qq<T>(to_email: String, content: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: lettre::message::IntoBody,
{
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    // 发件人邮箱和授权码
    let from_email = "aksjfds@qq.com"; // 替换为你的 QQ 邮箱
    let auth_code = "jgsqxzdvwumeebfa"; // 替换为你的授权码

    // 构建邮件
    let email = Message::builder()
        .from(from_email.parse()?)
        .to(to_email.parse().map_err(|_| "目标邮箱格式错误")?)
        .subject("MyStu 验证码")
        .header(ContentType::TEXT_PLAIN)
        .body(content)?;

    // 配置 SMTP 客户端
    let creds = Credentials::new(String::from(from_email), String::from(auth_code));
    let mailer = SmtpTransport::relay("smtp.qq.com")?
        .credentials(creds)
        .build();

    // 发送邮件
    mailer.send(&email).map_err(|_| "发送失败")?;

    Ok(())
}

