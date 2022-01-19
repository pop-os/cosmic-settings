use std::future::Future;

pub fn spawn_local<F: Future<Output = ()> + 'static>(future: F) {
	gtk4::glib::MainContext::default().spawn_local(future);
}
