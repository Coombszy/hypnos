package main

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/linde12/gowol"
)

// Request
type WOLRequest struct {
	MacAddress string `json:"macaddress"`
	Subnet     string `json:"subnet"`
}

// Handlers
func wakeSystem(c echo.Context) (err error) {

	// Bind data from context to object
	r := new(WOLRequest)
	if err := c.Bind(r); err != nil {
		fmt.Println(err.Error())
		return c.JSON(http.StatusBadRequest, err.Error())
	}

	// It the MacAddress or Subnet is 0 in size, return a bad request code
	if len(r.MacAddress) == 0 || len(r.Subnet) == 0 {
		return c.JSON(http.StatusBadRequest, "MacAddress/Subnet are length 0")
	}

	// Create a magic packet
	if packet, err := gowol.NewMagicPacket(r.MacAddress); err == nil {

		// Send packet using subnet
		if err := packet.Send(r.Subnet); err != nil {
			fmt.Println(err.Error())
			return c.JSON(http.StatusBadRequest, "Malformed Subnet: "+err.Error())
		}

	} else {
		fmt.Println(err.Error())
		return c.JSON(http.StatusBadRequest, "Malformed MacAddress: "+err.Error())
	}

	// Return if error
	if err != nil {
		fmt.Println(err.Error())
		return c.JSON(http.StatusInternalServerError, err)
	}

	// Returns
	return c.JSON(http.StatusOK, "Magic packet sent")
}

func main() {
	// Echo instance
	e := echo.New()

	// Middleware
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	// Routes
	e.POST("/", wakeSystem)

	// Start server
	e.Logger.Fatal(e.Start(":1323"))
}
