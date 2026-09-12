// UEFI Bootloader for LINIX OS
// 64-bit UEFI entry point

#include <efi.h>
#include <efilib.h>

// UEFI entry point
EFI_STATUS efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable) {
    InitializeLib(ImageHandle, SystemTable);
    
    // Clear screen
    uefi_call_wrapper(SystemTable->ConOut->ClearScreen, 1, SystemTable->ConOut);
    
    // Print welcome message
    Print(L"\n=== LINIX UEFI Bootloader ===\n");
    Print(L"UEFI 64-bit mode active\n");
    Print(L"Loading kernel...\n\n");
    
    // Get memory info
    Print(L"System Memory Info:\n");
    
    EFI_MEMORY_DESCRIPTOR *MemoryMap = NULL;
    UINTN MemoryMapSize = 0;
    UINTN MapKey;
    UINTN DescriptorSize;
    UINT32 DescriptorVersion;
    
    // Get memory map size
    uefi_call_wrapper(SystemTable->BootServices->GetMemoryMap, 5,
                      &MemoryMapSize, MemoryMap, &MapKey, &DescriptorSize, &DescriptorVersion);
    
    MemoryMapSize += 2 * DescriptorSize;
    
    // Allocate memory for map
    uefi_call_wrapper(SystemTable->BootServices->AllocatePool, 3,
                      EfiLoaderData, MemoryMapSize, (VOID **)&MemoryMap);
    
    // Get actual memory map
    uefi_call_wrapper(SystemTable->BootServices->GetMemoryMap, 5,
                      &MemoryMapSize, MemoryMap, &MapKey, &DescriptorSize, &DescriptorVersion);
    
    Print(L"Memory Map Size: %d bytes\n", MemoryMapSize);
    Print(L"Descriptor Size: %d bytes\n\n", DescriptorSize);
    
    // Load kernel file from disk
    Print(L"Loading kernel from disk...\n");
    
    // TODO: Implement kernel loading
    
    // For now, just wait for user input
    Print(L"\nPress any key to continue...\n");
    UINTN Index;
    uefi_call_wrapper(SystemTable->BootServices->WaitForEvent, 3, 1, &SystemTable->ConIn->WaitForKey, &Index);
    
    Print(L"Shutting down UEFI and jumping to kernel...\n");
    
    // Exit boot services
    uefi_call_wrapper(SystemTable->BootServices->ExitBootServices, 2, ImageHandle, MapKey);
    
    // Jump to kernel at 0x100000
    // TODO: Implement kernel jump
    
    return EFI_SUCCESS;
}
