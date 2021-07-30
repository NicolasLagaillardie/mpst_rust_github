/// Choose between many different sessions wrapped in an
/// `enum`
#[macro_export]
macro_rules! choose_tcp {
    ($label: path, $session: expr, $data: expr) => {{
        let (here, there) = <_ as Session>::new();
        let s = mpstthree::binary::send::send(($data, $label(there)), $session);
        mpstthree::binary::cancel::cancel(s);
        mpstthree::binary::cancel::cancel($data);
        here
    }};
}
