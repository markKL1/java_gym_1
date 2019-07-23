
FROM byrnedo/alpine-curl:latest

#RUN export http_proxy=proxy2.keylane.local:3128 && \
#    export shttp_proxy=proxy2.keylane.local:3128 && \
#    export HTTP_PROXY=proxy2.keylane.local:3128 && \
#    export SHTTP_PROXY=proxy2.keylane.local:3128

RUN ping -c1 8.8.8.8

RUN curl "https://www.google.com/"
