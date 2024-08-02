package ui.map

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.ktor.client.HttpClient
import io.ktor.client.call.body
import io.ktor.client.request.get
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable

class MapViewModel(
    initialNumber: Int,
    private val httpClient: HttpClient,
) : ViewModel() {
    private val _uiState = MutableStateFlow(initialNumber)
    val uiState: StateFlow<Int> = _uiState.asStateFlow()

    fun increase() {
        _uiState.value += 1
    }

    fun decrease() {
        _uiState.value -= 1
    }

    fun random() {
        viewModelScope.launch {
            val posts: List<Post> = withContext(Dispatchers.Default) {
                httpClient
                    .get("https://jsonplaceholder.typicode.com/posts")
                    .body()
            }

            withContext(Dispatchers.Main) {
                _uiState.value = posts.size
            }
        }
    }
}

@Serializable
data class Post(val id: Int, val title: String, val body: String, val userId: Int)
