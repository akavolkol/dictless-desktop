pre-build:
	glib-compile-resources --sourcedir resources --generate resources/bundle.xml \
	&& rm -rf target/debug/resources \
	&& cp -r resources target/debug/resources
