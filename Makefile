all:
	docker build --build-arg CACHEBUST=`git rev-parse master` -t crusts .
