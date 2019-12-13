#include <stdio.h>

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

#include "esp_system.h"
#include "esp_spi_flash.h"
#include "esp_task_wdt.h"

#include "driver/gpio.h"

#include <esp_log.h>

#include "sdkconfig.h"

static const char *TAG = "app_main";


// - rust imports -------------------------------------------------------------

extern void rust_main();


// - entry-point --------------------------------------------------------------

void app_main(void)
{
    // disable task watchdog
    TaskHandle_t idle_0 = xTaskGetIdleTaskHandleForCPU(0);
    if (idle_0 == NULL || esp_task_wdt_delete(idle_0) != ESP_OK){
        ESP_LOGI(TAG, "Failed to disable task watchdog timer for core 0");
    }
    TaskHandle_t idle_1 = xTaskGetIdleTaskHandleForCPU(1);
    if (idle_1 == NULL || esp_task_wdt_delete(idle_0) != ESP_OK) {
        ESP_LOGI(TAG, "Failed to disable task watchdog timer for core 1");
    }

    // dump chip information
    esp_chip_info_t chip_info;
    esp_chip_info(&chip_info);
    printf("This is an ESP32 chip with %d CPU cores, WiFi%s%s, ",
            chip_info.cores,
            (chip_info.features & CHIP_FEATURE_BT) ? "/BT" : "",
            (chip_info.features & CHIP_FEATURE_BLE) ? "/BLE" : "");
    printf("silicon revision %d, ", chip_info.revision);
    printf("%dMB %s flash\n", spi_flash_get_chip_size() / (1024 * 1024),
            (chip_info.features & CHIP_FEATURE_EMB_FLASH) ? "embedded" : "external");

    // call rust
    rust_main();

    // restart
    for (int i = 10; i >= 0; i--) {
        printf("Restarting in %d seconds...\n", i);
        vTaskDelay(1000 / portTICK_PERIOD_MS);
    }
    printf("Restarting now.\n");
    fflush(stdout);
    esp_restart();
}
