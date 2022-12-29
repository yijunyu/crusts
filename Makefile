all:
	docker build --build-arg CACHEBUST=`git rev-parse master` -t crusts .
	docker tag crusts yijun/crusts
	docker push yijun/crusts
