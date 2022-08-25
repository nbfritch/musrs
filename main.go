package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func serveSong(c http.ResponseWriter, r *http.Request) {
	http.ServeFile(c, r, "./public/song1.flac")
}

func main() {
	r := gin.Default()

	r.LoadHTMLGlob("templates/**/*")
	r.GET("/", func(ctx *gin.Context) {
		ctx.HTML(http.StatusOK, "index.page.html", nil)
	})

	r.GET("/music", gin.WrapF(serveSong))

	r.Static("/static", "./public")

	r.Run("0.0.0.0:3000")
}
