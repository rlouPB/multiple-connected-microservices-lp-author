(function() {
  const orderIdField = document.getElementById("order-id");
  const productIdField = document.getElementById("product-id");
  const quantityField = document.getElementById("quantity");
  const subtotalField = document.getElementById("subtotal");
  const shippingAddressField = document.getElementById("shippingAddress");
  const shippingZipField = document.getElementById("shippingZip");
  const totalField = document.getElementById("total");

  function displayError(err) {
    alert("Error:" + err);
  }

  async function onComputeButton() {
    const data = {
      order_id : parseFloat(orderIdField.value),
      product_id : parseFloat(productIdField.value),
      quantity : parseFloat(quantityField.value),
      subtotal : parseFloat(subtotalField.value),
      shipping_address : shippingAddressField.value,
      shipping_zip : shippingZipField.value,
      total : 0.0,
    };

    try {
      const response =  await fetch("http://localhost:8002/compute", {
        method: "POST",
        body: JSON.stringify(data),
        headers: { "Content-type": "application/json" },
      });
      if (!response.ok) {
        throw new Error(`HTTP error ${response.status}`);
      } else {
        if (response.status === 202) {
          const result = await response.json();
          alert(result.message);
        } else {
          const result = await response.json();
          console.log("Success:", result);
          updateOrderForm(result);  // Assume this is your UI update function
        }
      }
    } catch (error) {
      console.error("Fetch error:", error.message);
    }
  }
  function updateOrderForm(json) {
    alert("The order total for " + json.order_id + " has been updated to " + json.total);
    totalField.value = json.total;
  }

  document.getElementById("compute").addEventListener("click", onComputeButton);

})();
