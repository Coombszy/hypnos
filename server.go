package main

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	// NOTE(Liam)
	// Check on the following package, it could be a suitable go module for WOL:
	// https://github.com/migueleliasweb/go-wol
)

// Handlers
func wakeSystem(c echo.Context) (err error) {

	// Return if error
	if err != nil {
		fmt.Println("scan error : " + err.Error())
		return c.JSON(http.StatusInternalServerError, err)
	}

	// Returns
	return c.JSON(http.StatusOK, "TEST!")
}

func main() {
	// Echo instance
	e := echo.New()

	// Middleware
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	// Routes
	e.GET("/", wakeSystem)

	// Start server
	e.Logger.Fatal(e.Start(":1323"))
}
