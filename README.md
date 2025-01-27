# Diswh

A webhook api designed for use with the ESP platform. Relies on the esp-idf-svc library to handle networking.

Unlike the standard diswh, this version does *not* provide a `async` api. However given the ESP platform is underpowered, i believe its a compromize we can live with.

**REQUIRES `std` WILL NOT RUN ON `no_std`**

# How to use:

```rs
use diswh::{MessageBuilder, WebhookBuilder};

WebhookBuilder::new("url")
    .send_message(
        MessageBuilder::new("Hello webhook!", false).build()
    )?;
```