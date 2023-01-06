/// Peer message exchange schema
///
/// CLIENT                                            PEER
/// create a TCP connection                             ->
/// send handshake message                              ->
/// <---------                   receive handshake message
/// <---------                     receive unchoke message
/// send interested message                             ->
/// <----                 receive have or bitfield message
/// send request message                                ->
/// <-----------                 receive a requested piece
#[allow(dead_code)]
fn download() {}
