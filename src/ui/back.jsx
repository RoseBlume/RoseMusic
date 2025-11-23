
import { handleMenuClick } from "../lib/funcs";
export function Back() {
    return (
        <h2 class="Back" onClick={handleMenuClick}>&larr; Back</h2>
    );
}