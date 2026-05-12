# Explaining The Commit

## Experiment 2.1
![alt text](/images/broadcast_async.png)
Gambar: Running 3 Klien dan 1 Server <br>
Eksperimen ini mensimulasikan websocket, pada folder bin terdapat server dan client, server akan menerima pesan dari client dan kemudian mengirimkan pesan tersebut ke semua client yang terhubung. Pada gambar di atas, kita dapat melihat bahwa ketika salah satu client mengirimkan pesan, semua client lainnya menerima pesan tersebut secara bersamaan. <br>

Cara running:
- Buka 4 Terminal pada utama
- Pada terminal pertama, jalankan server dengan perintah `cargo run --bin server`
- Pada terminal kedua, jalankan client pertama dengan perintah `cargo run --bin client`
- Pada terminal ketiga, jalankan client kedua dengan perintah `cargo run --bin client`
- Pada terminal keempat, jalankan client ketiga dengan perintah `cargo run --bin client`
- Kemudian pada salah satu terminal client, ketik pesan dan tekan enter, maka semua client lainnya akan menerima pesan tersebut secara bersamaan. <br>

## Experiment 2.2
Eksperimen ini hanya mengganti port yang awalnya ada di port 2020 menjadi port 8080, perubahan code hanya ada di TCPListener pada server dan ClientBuilder pada client dengan menyesuaikan port 8080. <br>

Apakah file lain dari modul yaitu server.rs menggunakan ws protokol juga? ya, file server.rs menggunakan TCPListener untuk menerima koneksi dari client, lalu koneksi TCP tersebut dibungkus menjadi protokol ws (dapat dilihat di line 51, server.rs).